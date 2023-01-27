use crate::models::{
    display_mode_view_model::DisplayModeViewModel, 
    call_view_model::CallViewModel, 
    call_view_model::{CallsViewModel}, 
    destination_view_model::DestinationViewModel
};
use itxpt::{vehicle_journey_details::{VehicleJourneyDetails}, expected_call::ExpectedCall, vehicle_state::{VehicleState, self}};
use log::info;
use super::parse_itxpt::{GptParser};


#[derive(Clone)]
pub struct DataHandler {
    display_mode: Option<DisplayModeViewModel>,
    calls: Option<CallsViewModel>,
    destination: Option<DestinationViewModel>,

    vehicle_journey_details: Option<VehicleJourneyDetails>,
    expected_call: Option<ExpectedCall>,
    vehicle_state: Option<VehicleState>,
}

impl DataHandler {
    pub fn new() -> Self{
        DataHandler {
            display_mode: None,
            calls: None,
            destination: None,
            vehicle_journey_details: None,
            expected_call: None,
            vehicle_state: None,
        }
    }

    pub fn display_mode(&self) -> &Option<DisplayModeViewModel> { return &self.display_mode }
    pub fn calls(&self) -> &Option<CallsViewModel> { return &self.calls }
    pub fn destination(&self) -> &Option<DestinationViewModel> { return &self.destination }
    
    pub fn set_vehicle_journey_details(&mut self, vehicle_journey_details: VehicleJourneyDetails) {
        self.vehicle_journey_details = Some(vehicle_journey_details);
        self.calls = DataHandler::create_calls(&self.vehicle_journey_details, &self.expected_call);
        self.destination = DataHandler::create_destination(&self.vehicle_journey_details, &self.expected_call);
    }

    pub fn set_expected_calls(&mut self, expected_call: ExpectedCall) {
        self.expected_call = Some(expected_call);
        self.calls = DataHandler::create_calls(&self.vehicle_journey_details, &self.expected_call);
        self.destination = DataHandler::create_destination(&self.vehicle_journey_details, &self.expected_call);
    }
    pub fn set_vehicle_state(&mut self, vehicle_state: VehicleState) {
        self.vehicle_state = Some(vehicle_state);
        self.display_mode = DataHandler::create_display_mode(&self.vehicle_state);
    }

    fn create_calls(vehicle_journey_details: &Option<VehicleJourneyDetails>, expected_call: &Option<ExpectedCall>) -> Option<CallsViewModel> {
        if let Some(vehicle_journey_details) = vehicle_journey_details{
            if let Some(expected_call) = expected_call {
                return Some(GptParser::parse_calls(vehicle_journey_details,expected_call));
            }
        }
        None
    }
    fn create_display_mode(vehicle_state: &Option<VehicleState>) -> Option<DisplayModeViewModel> {
        if let Some(vehicle_state) = vehicle_state {
            return Some(GptParser::parse_display_mode(vehicle_state))
        }
        None
    }
    fn create_destination(vehicle_journey_details: &Option<VehicleJourneyDetails>, expected_call: &Option<ExpectedCall>) -> Option<DestinationViewModel> {
        if let Some(vehicle_journey_details) = vehicle_journey_details{
            if let Some(expected_call) = expected_call {
                return Some(GptParser::parse_destination_text(vehicle_journey_details, expected_call));
            }
        }
        None
    }
}



