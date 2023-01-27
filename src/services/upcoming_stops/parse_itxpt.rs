
use log::{debug};
use crate::models::{display_mode_view_model::DisplayModeViewModel, call_view_model::CallViewModel, call_view_model::CallsViewModel, destination_view_model::DestinationViewModel};
use itxpt::{
    expected_call::ExpectedCall, 
    vehicle_journey_details::VehicleJourneyDetails, 
    vehicle_journey_details::ArrivalType, 
    vehicle_state::VehicleState, 
    vehicle_state::DisplayMode,
};
use chrono::{DateTime, Utc, TimeZone};

pub struct GptParser {
}

impl GptParser {
    #[deprecated(since = "0.1.0", note = "Line color is a constant and not retrived for the specific vehicle.")]
    const LINE_COLOR: &str = "#C0030F";

    fn create_call_view_model(index: usize, ref_id: &str, estimated_time_of_arrival: DateTime<Utc>, vehicle_journey_details: &VehicleJourneyDetails, idx: &u32) -> Option<CallViewModel> {
        let details_call = &vehicle_journey_details.calls()[index];
        
        if let Some(stop_point) = details_call.stop_point() {
            let name = stop_point.name().unwrap_or("");
            let sequence_number = details_call.sequence_number();
            let drop_off = details_call.arrival().arrival_type() != &ArrivalType::StopNoAlighting;
            return Some(CallViewModel::new(
                ref_id.to_string(),
                name.to_string(),
                sequence_number,
                estimated_time_of_arrival,
                drop_off,
                *idx
            ));
        }
        return None;
    }

    pub fn parse_calls(vehicle_journey_details: &VehicleJourneyDetails, expected_call: &ExpectedCall) -> CallsViewModel {
        // TODO: Implement null and string checks for rust and logging.
    
        let index: usize = (expected_call.call_sequence_number()-1) as usize;
        let details_call = &vehicle_journey_details.calls()[index];
        
        let mut vec: Vec<CallViewModel> = Vec::new();
        let mut idx: u32 = 0;
        let estimated_time_of_arrival = expected_call.estimated_time_of_arrival().unwrap_or(Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap());
        if let Some(call_view_model) = GptParser::create_call_view_model(index, expected_call.point_ref(), estimated_time_of_arrival, vehicle_journey_details, &idx){
            vec.push(call_view_model);
        }
        if let Some(later_calls) = expected_call.later_calls() {
            for call in later_calls {
                idx += 1;
                let index: usize = (call.call_sequence_number()-1) as usize;
                let estimated_time_of_arrival = call.estimated_time_of_arrival().unwrap_or(Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap());
                if let Some(call_view_model) = GptParser::create_call_view_model(index, call.point_ref(), estimated_time_of_arrival, vehicle_journey_details, &idx){
                    vec.push(call_view_model);
                }
            }
        }
        return CallsViewModel::new(vec);
    }

    pub fn parse_display_mode(vehicle_state: &VehicleState) -> DisplayModeViewModel{
        let display_mode: DisplayMode = vehicle_state.display_mode().unwrap_or(DisplayMode::Unknown);
        return DisplayModeViewModel::new(display_mode as u32);
    }

    pub fn parse_destination_text(vehicle_journey_details: &VehicleJourneyDetails, expected_call: &ExpectedCall) -> DestinationViewModel {
        let calls = vehicle_journey_details.calls();
        let call_sequence_number = expected_call.call_sequence_number();
        for call in calls {
            if call_sequence_number == call.sequence_number() {
                if let Some(destination_display) = call.destination_display(){
                    let destination_display = DestinationViewModel::new(
                        destination_display.line_designation(),
                        destination_display.primary_destination(),
                        destination_display.secondary_destination().unwrap_or(vec![]),
                        GptParser::LINE_COLOR
                    );
                    return destination_display;
                }
                else {
                    debug!("Found correct call sequence number but missing 'destination_display' on call.");
                }                
            }
            else {
                debug!("Could not match the call sequence number in calls from details.");
            }
        } 
        debug!("Could not create a destination view model returning empty object.");
        DestinationViewModel::new(
            "".to_string(),
            "".to_string(),
            vec![],
            "".to_string()
        )
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        display_mode_view_model::DisplayModeViewModel, 
        call_view_model::CallsViewModel, 
        destination_view_model::DestinationViewModel
    };

    #[test]
    fn parse_itxpt_to_gpt_calls() {
        // Parse expected call and vehicle journey details from ITxPT that will be parsed into the GPT message for 'calls'
        let expected_call: Result<ExpectedCall, serde_json::Error> = serde_json::from_str(&GPT_EXPECTED_CALLS_MSG_INPUT);
        assert!(expected_call.is_ok());

        let vehicle_journey_details: Result<VehicleJourneyDetails, serde_json::Error> = serde_json::from_str(&GPT_DETAILS_MSG_INPUT);
        assert!(vehicle_journey_details.is_ok());  

        let parsed_calls: CallsViewModel = GptParser::parse_calls(&vehicle_journey_details.unwrap(), &expected_call.unwrap());          
        
        // ViewModels have a hash value generated when they are created from 'new'. To match correctly we use the generated hash for parsing the expected output.
        let gpt_calls_msg_output = GPT_CALLS_MSG_OUTPUT.replace("<hash-value>", parsed_calls.get_hash());
        let calls: Result<CallsViewModel, serde_json::Error> = serde_json::from_str(&gpt_calls_msg_output);

        assert!(calls.is_ok());  
        let calls: CallsViewModel = calls.unwrap();
        assert_eq!(calls, parsed_calls);
    }
    
    #[test]
    fn parse_itxpt_to_gpt_display_mode() {
        let vehicle_state: Result<VehicleState, serde_json::Error> = serde_json::from_str(&GPT_DISPLAY_MODE_MSG_INPUT);
        assert!(vehicle_state.is_ok());
        let parsed_display_mode: DisplayModeViewModel = GptParser::parse_display_mode(&vehicle_state.unwrap());

        let gpt_display_mode_msg_output = GPT_DISPLAY_MODE_MSG_OUTPUT.replace("<hash-value>", parsed_display_mode.get_hash());
        let display_mode: Result<DisplayModeViewModel, serde_json::Error> = serde_json::from_str(&gpt_display_mode_msg_output);
        assert!(display_mode.is_ok());

        let display_mode: DisplayModeViewModel = display_mode.unwrap();
        assert_eq!(display_mode, parsed_display_mode);
    }

    #[test]
    fn parse_itxpt_to_gpt_destination() {

        let expected_call: Result<ExpectedCall, serde_json::Error> = serde_json::from_str(&GPT_EXPECTED_CALLS_MSG_INPUT);
        assert!(expected_call.is_ok());
        let expected_call = expected_call.unwrap();

        let vehicle_journey_details: Result<VehicleJourneyDetails, serde_json::Error> = serde_json::from_str(&GPT_DETAILS_MSG_INPUT);
        assert!(vehicle_journey_details.is_ok());
        let vehicle_journey_details = vehicle_journey_details.unwrap();

        let parsed_destination_text = GptParser::parse_destination_text(&vehicle_journey_details, &expected_call);
        let gpt_destination_msg_output = GPT_DESTINATION_MSG_OUTPUT.replace("<hash-value>", parsed_destination_text.get_hash());
        
        let destination_display:Result<DestinationViewModel, serde_json::Error> = serde_json::from_str(&gpt_destination_msg_output);
        let destination_display = destination_display.unwrap();

        assert_eq!(destination_display, parsed_destination_text);
    }

    const GPT_CALLS_MSG_OUTPUT: &str = "{\"calls\":[{\"stopId\":\"9022005001027017\",\"stopName\":\"Strömbacken\",\"stopSequence\":9,\"arrivalTime\":\"2023-01-16T12:44:40Z\",\"dropOff\":true,\"index\":0},{\"stopId\":\"9022005000008017\",\"stopName\":\"Väster Tull\",\"stopSequence\":10,\"arrivalTime\":\"2023-01-16T12:46:00Z\",\"dropOff\":true,\"index\":1},{\"stopId\":\"9022005000003019\",\"stopName\":\"Söder Tull\",\"stopSequence\":11,\"arrivalTime\":\"2023-01-16T12:48:21Z\",\"dropOff\":true,\"index\":2},{\"stopId\":\"9022005000011017\",\"stopName\":\"Hörsalsparken\",\"stopSequence\":12,\"arrivalTime\":\"2023-01-16T12:51:13Z\",\"dropOff\":true,\"index\":3},{\"stopId\":\"9022005000005017\",\"stopName\":\"Rådhuset\",\"stopSequence\":13,\"arrivalTime\":\"2023-01-16T12:53:14Z\",\"dropOff\":true,\"index\":4},{\"stopId\":\"9022005001000017\",\"stopName\":\"Norrköpings resecentrum\",\"stopSequence\":14,\"arrivalTime\":\"2023-01-16T12:55:03Z\",\"dropOff\":true,\"index\":5},{\"stopId\":\"9022005000001017\",\"stopName\":\"Norr Tull\",\"stopSequence\":15,\"arrivalTime\":\"2023-01-16T12:56:16Z\",\"dropOff\":true,\"index\":6},{\"stopId\":\"9022005001002017\",\"stopName\":\"Matteusskolan\",\"stopSequence\":16,\"arrivalTime\":\"2023-01-16T12:57:55Z\",\"dropOff\":true,\"index\":7},{\"stopId\":\"9022005001020017\",\"stopName\":\"Marielund\",\"stopSequence\":17,\"arrivalTime\":\"2023-01-16T12:59:09Z\",\"dropOff\":true,\"index\":8},{\"stopId\":\"9022005000006017\",\"stopName\":\"Hagaskolan\",\"stopSequence\":18,\"arrivalTime\":\"2023-01-16T13:00:41Z\",\"dropOff\":true,\"index\":9},{\"stopId\":\"9022005001037017\",\"stopName\":\"Breda vägen\",\"stopSequence\":19,\"arrivalTime\":\"2023-01-16T13:02:07Z\",\"dropOff\":true,\"index\":10},{\"stopId\":\"9022005001038017\",\"stopName\":\"Sandbyhov\",\"stopSequence\":20,\"arrivalTime\":\"2023-01-16T13:03:34Z\",\"dropOff\":true,\"index\":11},{\"stopId\":\"9022005001039017\",\"stopName\":\"Rågången\",\"stopSequence\":21,\"arrivalTime\":\"2023-01-16T13:04:18Z\",\"dropOff\":true,\"index\":12},{\"stopId\":\"9022005000012017\",\"stopName\":\"Vidablick\",\"stopSequence\":22,\"arrivalTime\":\"2023-01-16T13:04:55Z\",\"dropOff\":true,\"index\":13}],\"hash\":\"<hash-value>\"}";
    const GPT_DISPLAY_MODE_MSG_OUTPUT: &str = "{\"mode\":1,\"hash\":\"<hash-value>\"}";
    const GPT_DESTINATION_MSG_OUTPUT: &str = "{\"routeName\":\"3\",\"text\":\"Vidablick\",\"alternativeText\":[],\"lineColor\":\"#C0030F\",\"hash\":\"<hash-value>\"}";

    const GPT_DISPLAY_MODE_MSG_INPUT: &str = "{\"_displayMode\":\"Normal\",\"_eventId\":\"5ef01976-156d-470c-92a6-2065d6981709\",\"_eventType\":\"mi/tdp/state\",\"_sequenceNumber\":11}";
    const GPT_DETAILS_MSG_INPUT: &str = "{\"operatingDayDate\":\"2023-01-16T00:00:00\",\"vehicleJourneyRef\":\"9015005000300104\",\"journeyNumber\":\"104\",\"transportModeCode\":\"TRAM\",\"transportAuthority\":{\"ref\":\"9010005000000000\",\"code\":\"OTN\",\"name\":\"Östgötatrafiken\",\"number\":\"5\"},\"plannedStartDateTime\":\"2023-01-16T12:34:00Z\",\"plannedEndDateTime\":\"2023-01-16T13:09:00Z\",\"origin\":{\"name\":\"Klockaretorpets vändplats\"},\"line\":{\"ref\":\"9011005000300000\",\"designation\":\"3\",\"number\":\"9011005000300000\"},\"directionOfLine\":{\"code\":\"0\",\"name\":\"\"},\"calls\":[{\"sequenceNumber\":1,\"journeyPatternPoint\":{\"isTimingPoint\":true},\"stopArea\":{\"ref\":\"9021005000010000\",\"name\":\"Klockaretorpets vändplats\"},\"stopPoint\":{\"ref\":\"9022005000010017\",\"name\":\"Klockaretorpets vändplats\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:34:00Z\",\"arrivalType\":\"STOP_NO_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:34:00Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":2,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001034000\",\"name\":\"Klockaretorpets centrum\"},\"stopPoint\":{\"ref\":\"9022005001034017\",\"name\":\"Klockaretorpets centrum\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:35:28Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:35:28Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":3,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001033000\",\"name\":\"Bastuban\"},\"stopPoint\":{\"ref\":\"9022005001033017\",\"name\":\"Bastuban\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:36:28Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:36:28Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":4,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001032000\",\"name\":\"SMHI\"},\"stopPoint\":{\"ref\":\"9022005001032017\",\"name\":\"SMHI\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:37:58Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:37:58Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":5,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005000009000\",\"name\":\"Folkets Park\"},\"stopPoint\":{\"ref\":\"9022005000009017\",\"name\":\"Folkets Park\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:38:58Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:38:58Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":6,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001031000\",\"name\":\"Skarphagsgatan\"},\"stopPoint\":{\"ref\":\"9022005001031017\",\"name\":\"Skarphagsgatan\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:40:00Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:40:00Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":7,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001030000\",\"name\":\"Lokegatan\"},\"stopPoint\":{\"ref\":\"9022005001030017\",\"name\":\"Lokegatan\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:41:00Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:41:00Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":8,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001029000\",\"name\":\"Vägträffen\"},\"stopPoint\":{\"ref\":\"9022005001029017\",\"name\":\"Vägträffen\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:42:36Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:42:36Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":9,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001027000\",\"name\":\"Strömbacken\"},\"stopPoint\":{\"ref\":\"9022005001027017\",\"name\":\"Strömbacken\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:43:52Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:43:52Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":10,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005000008000\",\"name\":\"Väster Tull\"},\"stopPoint\":{\"ref\":\"9022005000008017\",\"name\":\"Väster Tull\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:45:08Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:45:08Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":11,\"journeyPatternPoint\":{\"isTimingPoint\":true},\"stopArea\":{\"ref\":\"9021005000003000\",\"name\":\"Söder Tull\"},\"stopPoint\":{\"ref\":\"9022005000003019\",\"name\":\"Söder Tull\",\"designation\":\"G\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:49:00Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:50:00Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":12,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005000011000\",\"name\":\"Hörsalsparken\"},\"stopPoint\":{\"ref\":\"9022005000011017\",\"name\":\"Hörsalsparken\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:51:01Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:51:01Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":13,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005000005000\",\"name\":\"Rådhuset\"},\"stopPoint\":{\"ref\":\"9022005000005017\",\"name\":\"Rådhuset\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:52:43Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:52:43Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":14,\"journeyPatternPoint\":{\"isTimingPoint\":true},\"stopArea\":{\"ref\":\"9021005001000000\",\"name\":\"Norrköpings resecentrum\"},\"stopPoint\":{\"ref\":\"9022005001000017\",\"name\":\"Norrköpings resecentrum\",\"designation\":\"D2\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:55:00Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:55:00Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":15,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005000001000\",\"name\":\"Norr Tull\"},\"stopPoint\":{\"ref\":\"9022005000001017\",\"name\":\"Norr Tull\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:56:20Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:56:20Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":16,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001002000\",\"name\":\"Matteusskolan\"},\"stopPoint\":{\"ref\":\"9022005001002017\",\"name\":\"Matteusskolan\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:58:02Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:58:02Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":17,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001020000\",\"name\":\"Marielund\"},\"stopPoint\":{\"ref\":\"9022005001020017\",\"name\":\"Marielund\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T12:59:49Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T12:59:49Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":18,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005000006000\",\"name\":\"Hagaskolan\"},\"stopPoint\":{\"ref\":\"9022005000006017\",\"name\":\"Hagaskolan\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T13:01:32Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T13:01:32Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":19,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001037000\",\"name\":\"Breda vägen\"},\"stopPoint\":{\"ref\":\"9022005001037017\",\"name\":\"Breda vägen\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T13:03:21Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T13:03:21Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":20,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001038000\",\"name\":\"Sandbyhov\"},\"stopPoint\":{\"ref\":\"9022005001038017\",\"name\":\"Sandbyhov\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T13:04:56Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T13:04:56Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":21,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001039000\",\"name\":\"Rågången\"},\"stopPoint\":{\"ref\":\"9022005001039017\",\"name\":\"Rågången\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T13:06:16Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T13:06:16Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}},{\"sequenceNumber\":22,\"journeyPatternPoint\":{\"isTimingPoint\":true},\"stopArea\":{\"ref\":\"9021005000012000\",\"name\":\"Vidablick\"},\"stopPoint\":{\"ref\":\"9022005000012017\",\"name\":\"Vidablick\",\"designation\":\"B\"},\"arrival\":{\"latestDateTime\":\"2023-01-16T13:09:00Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-16T13:09:00Z\",\"departureType\":\"STOP_NO_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Vidablick\"}}}],\"gtfsShapeId\":\"2055700000054710361\",\"gtfsTripId\":\"55700000066534053\",\"gptShapeHash\":\"11CD688530C76E837D141D3C6B63EB96\",\"journeyPath\":[[58.57523727416992,16.125572204589844],[58.575199127197266,16.125486373901367],[58.57532501220703,16.125675201416016],[58.5755500793457,16.126039505004883],[58.57576370239258,16.126419067382812],[58.57604217529297,16.126937866210938],[58.57615661621094,16.127178192138672],[58.576210021972656,16.127317428588867],[58.57627487182617,16.12752342224121],[58.57634353637695,16.127798080444336],[58.57638931274414,16.128023147583008],[58.576416015625,16.128280639648438],[58.57642364501953,16.12845230102539],[58.57642364501953,16.129106521606445],[58.57645797729492,16.12944984436035],[58.57666015625,16.130346298217773],[58.57717514038086,16.132585525512695],[58.577335357666016,16.133291244506836],[58.57744216918945,16.133825302124023],[58.57807159423828,16.137405395507812],[58.578121185302734,16.137922286987305],[58.5782356262207,16.13897132873535],[58.57828140258789,16.139333724975586],[58.57840347290039,16.13995361328125],[58.57952880859375,16.145517349243164],[58.57960891723633,16.145912170410156],[58.579681396484375,16.146291732788086],[58.57969665527344,16.146514892578125],[58.579681396484375,16.146703720092773],[58.579654693603516,16.146875381469727],[58.5795783996582,16.14704704284668],[58.57945251464844,16.147235870361328],[58.57845687866211,16.148359298706055],[58.5781135559082,16.148752212524414],[58.57802200317383,16.14887046813965],[58.57798767089844,16.14893913269043],[58.57794952392578,16.149059295654297],[58.57793045043945,16.14916229248047],[58.57792282104492,16.14926528930664],[58.57793045043945,16.149850845336914],[58.57792663574219,16.150400161743164],[58.577938079833984,16.15052032470703],[58.57795333862305,16.15067481994629],[58.577980041503906,16.15081214904785],[58.57801818847656,16.15093421936035],[58.57831954956055,16.151796340942383],[58.57850646972656,16.152278900146484],[58.579505920410156,16.154644012451172],[58.579524993896484,16.154678344726562],[58.57978057861328,16.15533447265625],[58.58012008666992,16.156129837036133],[58.58045196533203,16.156871795654297],[58.58101272583008,16.158199310302734],[58.58115768432617,16.158597946166992],[58.581298828125,16.159130096435547],[58.58138656616211,16.15945816040039],[58.581451416015625,16.159631729125977],[58.58147430419922,16.159700393676758],[58.581520080566406,16.15976905822754],[58.58163833618164,16.159873962402344],[58.584197998046875,16.162015914916992],[58.58488845825195,16.16260528564453],[58.58497619628906,16.16272735595703],[58.58504867553711,16.1628475189209],[58.585113525390625,16.163021087646484],[58.58516311645508,16.16326141357422],[58.585243225097656,16.163658142089844],[58.58597946166992,16.167139053344727],[58.58603286743164,16.167482376098633],[58.5860595703125,16.16765594482422],[58.58610153198242,16.168275833129883],[58.58613967895508,16.168550491333008],[58.586181640625,16.168739318847656],[58.58622741699219,16.168930053710938],[58.586421966552734,16.169618606567383],[58.58650207519531,16.169981002807617],[58.58656311035156,16.170377731323242],[58.58672332763672,16.171375274658203],[58.58673858642578,16.171669006347656],[58.58673095703125,16.171995162963867],[58.58649826049805,16.174726486206055],[58.586387634277344,16.175670623779297],[58.586238861083984,16.17742347717285],[58.586238861083984,16.177440643310547],[58.58617401123047,16.178298950195312],[58.58600616455078,16.180326461791992],[58.585941314697266,16.181201934814453],[58.58538055419922,16.187780380249023],[58.58534622192383,16.188175201416016],[58.585289001464844,16.188812255859375],[58.585243225097656,16.189207077026367],[58.585235595703125,16.189361572265625],[58.585243225097656,16.189498901367188],[58.58528518676758,16.189619064331055],[58.585350036621094,16.189706802368164],[58.58542251586914,16.189741134643555],[58.58551025390625,16.189708709716797],[58.5858154296875,16.189556121826172],[58.58604049682617,16.18943977355957],[58.5862922668457,16.18927001953125],[58.587650299072266,16.188579559326172],[58.587947845458984,16.188461303710938],[58.58834457397461,16.18825912475586],[58.591453552246094,16.186660766601562],[58.59157943725586,16.186609268188477],[58.59180450439453,16.186527252197266],[58.592567443847656,16.186140060424805],[58.592613220214844,16.186105728149414],[58.59284973144531,16.185970306396484],[58.59446716308594,16.184974670410156],[58.59593963623047,16.184215545654297],[58.5960578918457,16.184114456176758],[58.596092224121094,16.184062957763672],[58.59614181518555,16.1839599609375],[58.59617614746094,16.183788299560547],[58.59617614746094,16.183650970458984],[58.59617614746094,16.183565139770508],[58.59599304199219,16.1822566986084],[58.59589385986328,16.181617736816406],[58.59583282470703,16.1812744140625],[58.59563064575195,16.180601119995117],[58.595558166503906,16.18029022216797],[58.595054626464844,16.176759719848633],[58.59497833251953,16.17624282836914],[58.594783782958984,16.17486572265625],[58.59407043457031,16.169767379760742],[58.59404373168945,16.16959571838379],[58.593868255615234,16.168561935424805],[58.5938606262207,16.168493270874023],[58.593868255615234,16.168371200561523],[58.593875885009766,16.168304443359375],[58.593894958496094,16.168235778808594],[58.59394073486328,16.168149948120117],[58.59397888183594,16.16809844970703],[58.594120025634766,16.167997360229492],[58.59705352783203,16.166479110717773],[58.59745788574219,16.166276931762695],[58.597572326660156,16.166175842285156],[58.597618103027344,16.16612434387207],[58.59804916381836,16.165578842163086],[58.598140716552734,16.16545867919922],[58.59821319580078,16.16533851623535],[58.59827423095703,16.16520118713379],[58.598331451416016,16.16504669189453],[58.59870910644531,16.163846969604492],[58.599578857421875,16.161104202270508],[58.59968185424805,16.160795211791992],[58.599735260009766,16.160675048828125],[58.59977722167969,16.160608291625977],[58.599815368652344,16.16059112548828],[58.599876403808594,16.160573959350586],[58.59996795654297,16.160627365112305],[58.600738525390625,16.161426544189453],[58.60188293457031,16.162555694580078],[58.60204315185547,16.162694931030273],[58.60268020629883,16.163217544555664],[58.60285186767578,16.163339614868164],[58.60323715209961,16.163497924804688],[58.603416442871094,16.1635684967041],[58.60357666015625,16.163604736328125],[58.60424041748047,16.163610458374023],[58.6043586730957,16.163578033447266],[58.60447692871094,16.16349220275879],[58.604576110839844,16.16335678100586],[58.60462951660156,16.163253784179688],[58.60468292236328,16.16313362121582],[58.604736328125,16.16291046142578],[58.604766845703125,16.16280746459961],[58.60478210449219,16.162635803222656],[58.60478210449219,16.162395477294922],[58.60464859008789,16.159212112426758],[58.6046142578125,16.15838623046875],[58.60460662841797,16.158109664916992],[58.604618072509766,16.157869338989258],[58.604652404785156,16.15764617919922],[58.604698181152344,16.157440185546875],[58.60477066040039,16.15723419189453],[58.60480880737305,16.157148361206055],[58.60490417480469,16.156978607177734],[58.605979919433594,16.1553897857666],[58.6063117980957,16.15489387512207],[58.6078987121582,16.15253448486328],[58.60871124267578,16.1513729095459],[58.60871887207031,16.151390075683594]],\"_eventId\":\"f93e4920-04a1-4f9c-a27d-7b49554c2626\",\"_eventType\":\"oi/current_vehicle_journey/details\",\"_sequenceNumber\":33}";
    const GPT_EXPECTED_CALLS_MSG_INPUT: &str = "{\"updatedAtDateTime\":\"2023-01-16T11:43:47Z\",\"vehicleJourneyRef\":\"9015005000300104\",\"callSequenceNumber\":9,\"pointRef\":\"9022005001027017\",\"atStop\":false,\"estimatedTimeOfArrival\":\"2023-01-16T12:44:40Z\",\"estimatedTimeOfDeparture\":\"2023-01-16T12:44:50Z\",\"serviceDeviation\":37,\"previousCall\":{\"callSequenceNumber\":8,\"pointRef\":\"9022005001029017\",\"estimatedTimeOfArrival\":\"2023-01-16T12:43:15Z\",\"estimatedTimeOfDeparture\":\"2023-01-16T12:43:15Z\",\"restriction\":\"UNKNOWN\"},\"laterCalls\":[{\"callSequenceNumber\":10,\"pointRef\":\"9022005000008017\",\"estimatedTimeOfArrival\":\"2023-01-16T12:46:00Z\",\"estimatedTimeOfDeparture\":\"2023-01-16T12:46:19Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":11,\"pointRef\":\"9022005000003019\",\"estimatedTimeOfArrival\":\"2023-01-16T12:48:21Z\",\"estimatedTimeOfDeparture\":\"2023-01-16T12:50:00Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":12,\"pointRef\":\"9022005000011017\",\"estimatedTimeOfArrival\":\"2023-01-16T12:51:13Z\",\"estimatedTimeOfDeparture\":\"2023-01-16T12:51:34Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":13,\"pointRef\":\"9022005000005017\",\"estimatedTimeOfArrival\":\"2023-01-16T12:53:14Z\",\"estimatedTimeOfDeparture\":\"2023-01-16T12:53:31Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":14,\"pointRef\":\"9022005001000017\",\"estimatedTimeOfArrival\":\"2023-01-16T12:55:03Z\",\"estimatedTimeOfDeparture\":\"2023-01-16T12:55:24Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":15,\"pointRef\":\"9022005000001017\",\"estimatedTimeOfArrival\":\"2023-01-16T12:56:16Z\",\"estimatedTimeOfDeparture\":\"2023-01-16T12:56:49Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":16,\"pointRef\":\"9022005001002017\",\"estimatedTimeOfArrival\":\"2023-01-16T12:57:55Z\",\"estimatedTimeOfDeparture\":\"2023-01-16T12:58:00Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":17,\"pointRef\":\"9022005001020017\",\"estimatedTimeOfArrival\":\"2023-01-16T12:59:09Z\",\"estimatedTimeOfDeparture\":\"2023-01-16T12:59:21Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":18,\"pointRef\":\"9022005000006017\",\"estimatedTimeOfArrival\":\"2023-01-16T13:00:41Z\",\"estimatedTimeOfDeparture\":\"2023-01-16T13:00:54Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":19,\"pointRef\":\"9022005001037017\",\"estimatedTimeOfArrival\":\"2023-01-16T13:02:07Z\",\"estimatedTimeOfDeparture\":\"2023-01-16T13:02:19Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":20,\"pointRef\":\"9022005001038017\",\"estimatedTimeOfArrival\":\"2023-01-16T13:03:34Z\",\"estimatedTimeOfDeparture\":\"2023-01-16T13:03:34Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":21,\"pointRef\":\"9022005001039017\",\"estimatedTimeOfArrival\":\"2023-01-16T13:04:18Z\",\"estimatedTimeOfDeparture\":\"2023-01-16T13:04:21Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":22,\"pointRef\":\"9022005000012017\",\"estimatedTimeOfArrival\":\"2023-01-16T13:04:55Z\",\"estimatedTimeOfDeparture\":\"2023-01-16T13:06:16Z\",\"restriction\":\"UNKNOWN\"}],\"gtfsTripId\":\"55700000066534053\",\"_eventId\":\"42935f18-eafc-4b11-9508-09975395de7e\",\"_eventType\":\"oi/current_vehicle_journey/expected_call\",\"_sequenceNumber\":1175}";
}
