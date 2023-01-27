
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ExpectedCall {
    #[serde(rename = "updatedAtDateTime")]
    updated_at_date_time: DateTime<Utc>,
    #[serde(rename = "vehicleJourneyRef")]
    vehicle_journey_ref: String,
    #[serde(rename = "callSequenceNumber")]
    call_sequence_number: u32,
    #[serde(rename = "pointRef")]
    point_ref: String,
    #[serde(rename = "atStop")]
    at_stop: bool,
    #[serde(rename = "estimatedTimeOfArrival")]
    estimated_time_of_arrival: Option<DateTime<Utc>>,
    #[serde(rename = "observedTimeOfArrival")]
    observed_time_of_arrival: Option<DateTime<Utc>>,
    #[serde(rename = "estimatedTimeOfDeparture")]
    estimated_time_of_departure: Option<DateTime<Utc>>,
    #[serde(rename = "serviceDeviation")]
    service_deviation: i32,
    #[serde(rename = "dutyDeviation")]
    duty_deviation: Option<i32>,
    #[serde(rename = "holdReason")]
    hold_reason: Option<HoldReasonEnum>,
    #[serde(rename = "holdUntil")]
    hold_until: Option<String>,
    #[serde(rename = "restriction")]
    restriction: Option<RestrictionEnum>,
    #[serde(rename = "previousCall")]
    previous_call: Option<Call>,
    #[serde(rename = "laterCalls")]
    later_calls: Option<Vec<Call>>,

    #[serde(rename = "gtfsTripId")]
    gtfs_trip_id: Option<String>,
    #[serde(rename = "_eventId")]
    event_id: Option<String>,
    #[serde(rename = "_eventType")]
    event_type: Option<String>,
    #[serde(rename = "_sequenceNumber")]
    sequnce_number: Option<i32>
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum HoldReasonEnum {
    #[serde(rename = "UNKNOWN")]
    Unknown = 0,
    #[serde(rename = "CONNECTION_PROTECTION")]
    ConnectionProtection = 1,
    #[serde(rename = "TIMING_POINT")]
    TimingPoint = 2,
    #[serde(rename = "DRIVER_CHANGE")]
    DriverChange = 3
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum RestrictionEnum {
    #[serde(rename = "UNKNOWN")]
    Unknown = 0,
    #[serde(rename = "NO_BOARDING")]
    NoBoarding = 1,
    #[serde(rename = "NO_ALIGHTING")]
    NoAlighting = 2,
    #[serde(rename = "NO_STOP")]
    NoStop = 3
}

#[allow(dead_code)]
impl ExpectedCall {
    // pub fn new (updated_at_date_time: DateTime<Utc>, vehicle_journey_ref: String, call_sequence_number: u32, point_ref: String, 
    //     at_stop: bool, estimated_time_of_arrival: Option<DateTime<Utc>>, observed_time_of_arrival: Option<DateTime<Utc>>, estimated_time_of_departure: Option<DateTime<Utc>>,
    //     service_deviation: i32, duty_deviation: Option<i32>, hold_reason: Option<HoldReasonEnum>, hold_until: Option<String>, restriction: Option<RestrictionEnum>, previous_call: Option<Call>,
    //     later_calls: Option<Vec<Call>>, gtfs_trip_id: Option<String>, event_id: Option<String>, event_type: Option<String>, sequnce_number: Option<i32>) -> Self {
    //     ExpectedCall {
    //         updated_at_date_time,
    //         vehicle_journey_ref,
    //         call_sequence_number,
    //         point_ref,
    //         at_stop,
    //         estimated_time_of_arrival,
    //         observed_time_of_arrival,
    //         estimated_time_of_departure,
    //         service_deviation,
    //         duty_deviation,
    //         hold_reason,
    //         hold_until,
    //         restriction,
    //         previous_call,
    //         later_calls,
    //         gtfs_trip_id,
    //         event_id,
    //         event_type,
    //         sequnce_number
    //     }
    // }

    pub fn point_ref(&self) -> &str { &self.point_ref.as_str() }

    pub fn call_sequence_number(&self) -> u32 { self.call_sequence_number }

    pub fn updated_at_date_time(&self) -> &DateTime<Utc> { &self.updated_at_date_time }

    pub fn estimated_time_of_arrival(&self) -> &Option<DateTime<Utc>> { &self.estimated_time_of_arrival }

    pub fn estimated_time_of_departure(&self) -> &Option<DateTime<Utc>> { &self.estimated_time_of_departure }

    pub fn observed_time_of_arrival(&self) -> &Option<DateTime<Utc>> { &self.observed_time_of_arrival }

    pub fn service_deviation(&self) -> i32 { self.service_deviation }

    pub fn hold_reason(&self) -> &Option<HoldReasonEnum> { &self.hold_reason }

    pub fn restriction(&self) -> &Option<RestrictionEnum> { &self.restriction }

    pub fn previous_call(&self) -> &Option<Call> { &self.previous_call }

    pub fn later_calls(&self) -> &Option<Vec<Call>> { &self.later_calls }

    pub fn gtfs_trip_id(&self) -> Option<&str> {
        if let Some(gtfs_trip_id_ret) = &self.gtfs_trip_id{
            return Some(&gtfs_trip_id_ret.as_str())
        }
        return None;
    }

    pub fn event_id(&self) -> Option<&str> {
        if let Some(event_id_ret) = &self.event_id{
            return Some(&event_id_ret.as_str())
        }
        return None;
    }

    pub fn event_type(&self) -> Option<&str> {
        if let Some(event_type_ret) = &self.event_type{
            return Some(&event_type_ret.as_str())
        }
        return None;
    }

    pub fn sequnce_number(&self) -> Option<&i32> {
        if let Some(sequnce_number_ret) = &self.sequnce_number{
            return Some(sequnce_number_ret)
        }
        return None;
    }    
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Call {
    #[serde(rename = "callSequenceNumber")]
    call_sequence_number: u32,
    #[serde(rename = "pointRef")]
    point_ref: String,
    #[serde(rename = "estimatedTimeOfArrival")]
    estimated_time_of_arrival: Option<DateTime<Utc>>,
    #[serde(rename = "observedTimeOfArrival")]
    observed_time_of_arrival: Option<DateTime<Utc>>,
    #[serde(rename = "estimatedTimeOfDeparture")]
    estimated_time_of_departure: Option<DateTime<Utc>>,
    #[serde(rename = "observedTimeOfDeparture")]
    observed_time_of_departure: Option<DateTime<Utc>>,
    #[serde(rename = "restriction")]
    restriction: Option<RestrictionEnum>
}

impl Call {
    pub fn new(call_sequence_number: u32, point_ref: String, estimated_time_of_arrival: Option<DateTime<Utc>>, observed_time_of_arrival: Option<DateTime<Utc>>,
            estimated_time_of_departure: Option<DateTime<Utc>>, observed_time_of_departure: Option<DateTime<Utc>>, restriction: Option<RestrictionEnum>) -> Self{
        Call {
            call_sequence_number,
            point_ref, 
            estimated_time_of_arrival,
            observed_time_of_arrival,
            estimated_time_of_departure,
            observed_time_of_departure,
            restriction
        }
    }

    pub fn call_sequence_number(&self) -> u32  { self.call_sequence_number }

    pub fn point_ref(&self) -> &str  { &self.point_ref.as_str() }

    pub fn estimated_time_of_arrival(&self) -> &Option<DateTime<Utc>>  { &self.estimated_time_of_arrival }


}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    
    #[test]
    fn deserialize_json_string_gpt() {
        let expected_call: Result<ExpectedCall, serde_json::Error> = serde_json::from_str(&GPT_MQTT_MSG);
        assert!(expected_call.is_ok());
    }
    #[test]
    fn verify_data_gpt() {
        let expected_call: Result<ExpectedCall, serde_json::Error> = serde_json::from_str(&GPT_MQTT_MSG);
        assert!(expected_call.is_ok());

        let expected_call = expected_call.unwrap();

        let service_deviation = 56;
        assert_eq!(expected_call.service_deviation, service_deviation);

        let updated_at_date_time = DateTime::parse_from_rfc3339("2023-01-12T07:36:05Z").unwrap().with_timezone(&Utc);
        assert_eq!(expected_call.updated_at_date_time(), &updated_at_date_time);

        let estimated_time_of_departure = Some(DateTime::parse_from_rfc3339("2023-01-12T08:35:47Z").unwrap().with_timezone(&Utc));
        assert_eq!(expected_call.estimated_time_of_departure(), &estimated_time_of_departure);

        let hold_reason = None;
        assert_eq!(expected_call.hold_reason(), &hold_reason);
        
        let restriction = None;
        assert_eq!(expected_call.restriction(), &restriction);

        let previous_call = Some(
            Call 
            { 
                call_sequence_number: 1, 
                point_ref: "9022005000010017".to_string(), 
                observed_time_of_arrival: None,
                observed_time_of_departure: None,
                restriction: Some(RestrictionEnum::Unknown),
                estimated_time_of_arrival: Some(DateTime::parse_from_rfc3339("2023-01-12T08:33:52Z").unwrap().with_timezone(&Utc)),
                estimated_time_of_departure: Some(DateTime::parse_from_rfc3339("2023-01-12T08:34:56Z").unwrap().with_timezone(&Utc)),
            }
        );
        assert_eq!(expected_call.previous_call(), &previous_call);

        
        let previous_call = Some(
            Call 
            { 
                call_sequence_number: 1, 
                point_ref: "9022005000010017".to_string(), 
                observed_time_of_arrival: None,
                observed_time_of_departure: None, 
                restriction: Some(RestrictionEnum::Unknown),
                estimated_time_of_arrival: Some(DateTime::parse_from_rfc3339("2023-01-12T08:33:52Z").unwrap().with_timezone(&Utc)),
                estimated_time_of_departure: Some(DateTime::parse_from_rfc3339("2023-01-12T08:34:56Z").unwrap().with_timezone(&Utc))
            }
        );
        assert_eq!(expected_call.previous_call(), &previous_call);
        
        let vec_length = 20;
        if let Some(vec) = expected_call.later_calls() {
            assert_eq!(vec.len(), vec_length);
        }

        let first_call = Some(
            Call 
            { 
                call_sequence_number: 3, 
                point_ref: "9022005001033017".to_string(), 
                observed_time_of_arrival: None,
                observed_time_of_departure: None, 
                restriction: Some(RestrictionEnum::Unknown),
                estimated_time_of_arrival: Some(DateTime::parse_from_rfc3339("2023-01-12T08:36:35Z").unwrap().with_timezone(&Utc)),
                estimated_time_of_departure: Some(DateTime::parse_from_rfc3339("2023-01-12T08:36:45Z").unwrap().with_timezone(&Utc))
            }
        );
        let last_call = Some(
            Call 
            { 
                call_sequence_number: 22, 
                point_ref: "9022005000012017".to_string(), 
                observed_time_of_arrival: None,
                observed_time_of_departure: None, 
                restriction: Some(RestrictionEnum::Unknown),
                estimated_time_of_arrival: Some(DateTime::parse_from_rfc3339("2023-01-12T09:04:28Z").unwrap().with_timezone(&Utc)),
                estimated_time_of_departure: Some(DateTime::parse_from_rfc3339("2023-01-12T09:06:16Z").unwrap().with_timezone(&Utc))
            }
        );

        if let Some(vec) = expected_call.later_calls() {
            assert_eq!(Some(vec[0].clone()), first_call);
            assert_eq!(Some(vec[19].clone()), last_call);
        }

        assert_eq!(expected_call.gtfs_trip_id(), Some("55700000066533349"));
        assert_eq!(expected_call.event_id(), Some("4014df07-f2c5-4c81-9d38-bdce9a959737"));
        assert_eq!(expected_call.event_type(), Some("oi/current_vehicle_journey/expected_call"));
        assert_eq!(expected_call.sequnce_number(), Some(&224));
    }

    #[test]
    fn deserialize_json_string_itxpt() {
        let expected_call: Result<ExpectedCall, serde_json::Error> = serde_json::from_str(&ITXPT_EXAMPLE_MSG);
        assert!(expected_call.is_ok());
    }
    #[test]
    fn verify_data_itxpt() {
        let expected_call: Result<ExpectedCall, serde_json::Error> = serde_json::from_str(&ITXPT_EXAMPLE_MSG);
        assert!(expected_call.is_ok());

        let expected_call = expected_call.unwrap();

        let service_deviation = -30;
        assert_eq!(expected_call.service_deviation(), service_deviation);

        let updated_at_date_time = DateTime::parse_from_rfc3339("2017-10-31T12:45:50Z").unwrap().with_timezone(&Utc);
        assert_eq!(expected_call.updated_at_date_time(), &updated_at_date_time);

        let estimated_time_of_departure = Some(DateTime::parse_from_rfc3339("2017-10-31T12:46:30Z").unwrap().with_timezone(&Utc));
        assert_eq!(expected_call.estimated_time_of_departure(), &estimated_time_of_departure);

        let hold_reason = Some(HoldReasonEnum::ConnectionProtection);
        assert_eq!(expected_call.hold_reason(), &hold_reason);
        
        let restriction = Some(RestrictionEnum::NoAlighting);
        assert_eq!(expected_call.restriction(), &restriction);

        let previous_call = Some(
            Call 
            { 
                call_sequence_number: 2, 
                point_ref: "9025001000012333".to_string(), 
                observed_time_of_arrival: Some(DateTime::parse_from_rfc3339("2017-10-31T12:43:50Z").unwrap().with_timezone(&Utc)),
                observed_time_of_departure: Some(DateTime::parse_from_rfc3339("2017-10-31T12:44:32Z").unwrap().with_timezone(&Utc)), 
                restriction: None,
                estimated_time_of_arrival: None,
                estimated_time_of_departure: None
            }
        );
        assert_eq!(expected_call.previous_call(), &previous_call);

        let mut later_calls_vec = Vec::new();
        later_calls_vec.push(
            Call
            {
                call_sequence_number: 4, 
                point_ref: "9025001000012377".to_string(), 
                observed_time_of_arrival: None,
                observed_time_of_departure: None, 
                restriction: Some(RestrictionEnum::NoBoarding),
                estimated_time_of_arrival: Some(DateTime::parse_from_rfc3339("2017-10-31T12:48:23Z").unwrap().with_timezone(&Utc)),
                estimated_time_of_departure: Some(DateTime::parse_from_rfc3339("2017-10-31T12:49:10Z").unwrap().with_timezone(&Utc))                
            }
        );
        later_calls_vec.push(
            Call
            {
                call_sequence_number: 5, 
                point_ref: "9025001000012332".to_string(), 
                observed_time_of_arrival: None,
                observed_time_of_departure: None, 
                restriction: None,
                estimated_time_of_arrival: Some(DateTime::parse_from_rfc3339("2017-10-31T12:53:50Z").unwrap().with_timezone(&Utc)),
                estimated_time_of_departure: Some(DateTime::parse_from_rfc3339("2017-10-31T12:54:20Z").unwrap().with_timezone(&Utc))                 
            }
        );
        let later_calls = Some(later_calls_vec);
        assert_eq!(expected_call.later_calls, later_calls);
        
    }

    const GPT_MQTT_MSG: &str = "{\"updatedAtDateTime\":\"2023-01-12T07:36:05Z\",\"vehicleJourneyRef\":\"9015005000300056\",\"callSequenceNumber\":2,\"pointRef\":\"9022005001034017\",\"atStop\":true,\"estimatedTimeOfArrival\":\"2023-01-12T08:35:47Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:35:47Z\",\"serviceDeviation\":56,\"previousCall\":{\"callSequenceNumber\":1,\"pointRef\":\"9022005000010017\",\"estimatedTimeOfArrival\":\"2023-01-12T08:33:52Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:34:56Z\",\"restriction\":\"UNKNOWN\"},\"laterCalls\":[{\"callSequenceNumber\":3,\"pointRef\":\"9022005001033017\",\"estimatedTimeOfArrival\":\"2023-01-12T08:36:35Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:36:45Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":4,\"pointRef\":\"9022005001032017\",\"estimatedTimeOfArrival\":\"2023-01-12T08:37:57Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:38:04Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":5,\"pointRef\":\"9022005000009017\",\"estimatedTimeOfArrival\":\"2023-01-12T08:39:02Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:39:07Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":6,\"pointRef\":\"9022005001031017\",\"estimatedTimeOfArrival\":\"2023-01-12T08:40:10Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:40:14Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":7,\"pointRef\":\"9022005001030017\",\"estimatedTimeOfArrival\":\"2023-01-12T08:40:51Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:40:56Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":8,\"pointRef\":\"9022005001029017\",\"estimatedTimeOfArrival\":\"2023-01-12T08:42:20Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:42:29Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":9,\"pointRef\":\"9022005001027017\",\"estimatedTimeOfArrival\":\"2023-01-12T08:43:51Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:43:58Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":10,\"pointRef\":\"9022005000008017\",\"estimatedTimeOfArrival\":\"2023-01-12T08:45:15Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:45:27Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":11,\"pointRef\":\"9022005000003019\",\"estimatedTimeOfArrival\":\"2023-01-12T08:48:36Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:50:00Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":12,\"pointRef\":\"9022005000011017\",\"estimatedTimeOfArrival\":\"2023-01-12T08:51:18Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:51:35Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":13,\"pointRef\":\"9022005000005017\",\"estimatedTimeOfArrival\":\"2023-01-12T08:53:08Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:53:21Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":14,\"pointRef\":\"9022005001000017\",\"estimatedTimeOfArrival\":\"2023-01-12T08:54:53Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:55:09Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":15,\"pointRef\":\"9022005000001017\",\"estimatedTimeOfArrival\":\"2023-01-12T08:56:07Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:56:47Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":16,\"pointRef\":\"9022005001002017\",\"estimatedTimeOfArrival\":\"2023-01-12T08:57:50Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:57:57Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":17,\"pointRef\":\"9022005001020017\",\"estimatedTimeOfArrival\":\"2023-01-12T08:59:04Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T08:59:10Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":18,\"pointRef\":\"9022005000006017\",\"estimatedTimeOfArrival\":\"2023-01-12T09:00:13Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T09:00:44Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":19,\"pointRef\":\"9022005001037017\",\"estimatedTimeOfArrival\":\"2023-01-12T09:01:36Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T09:01:46Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":20,\"pointRef\":\"9022005001038017\",\"estimatedTimeOfArrival\":\"2023-01-12T09:02:45Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T09:02:48Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":21,\"pointRef\":\"9022005001039017\",\"estimatedTimeOfArrival\":\"2023-01-12T09:03:47Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T09:03:50Z\",\"restriction\":\"UNKNOWN\"},{\"callSequenceNumber\":22,\"pointRef\":\"9022005000012017\",\"estimatedTimeOfArrival\":\"2023-01-12T09:04:28Z\",\"estimatedTimeOfDeparture\":\"2023-01-12T09:06:16Z\",\"restriction\":\"UNKNOWN\"}],\"gtfsTripId\":\"55700000066533349\",\"_eventId\":\"4014df07-f2c5-4c81-9d38-bdce9a959737\",\"_eventType\":\"oi/current_vehicle_journey/expected_call\",\"_sequenceNumber\":224}";
    const ITXPT_EXAMPLE_MSG: &str = "{\"updatedAtDateTime\":\"2017-10-31T12:45:50Z\",\"vehicleJourneyRef\":\"9015001098702345\",\"callSequenceNumber\":3,\"pointRef\":\"9025001000012345\",\"atStop\":true,\"observedTimeOfArrival\":\"2017-10-31T12:45:50Z\",\"estimatedTimeOfDeparture\":\"2017-10-31T12:46:30Z\",\"serviceDeviation\":-30,\"holdReason\":\"CONNECTION_PROTECTION\",\"holdUntil\":\"2017-10-31T12:46:30Z\",\"restriction\":\"NO_ALIGHTING\",\"previousCall\":{\"callSequenceNumber\":2,\"pointRef\":\"9025001000012333\",\"observedTimeOfArrival\":\"2017-10-31T12:43:50Z\",\"observedTimeOfDeparture\":\"2017-10-31T12:44:32Z\"},\"laterCalls\":[{\"callSequenceNumber\":4,\"pointRef\":\"9025001000012377\",\"estimatedTimeOfArrival\":\"2017-10-31T12:48:23Z\",\"estimatedTimeOfDeparture\":\"2017-10-31T12:49:10Z\",\"restriction\":\"NO_BOARDING\"},{\"callSequenceNumber\":5,\"pointRef\":\"9025001000012332\",\"estimatedTimeOfArrival\":\"2017-10-31T12:53:50Z\",\"estimatedTimeOfDeparture\":\"2017-10-31T12:54:20Z\"}]}";
}


