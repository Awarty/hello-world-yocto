use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
pub enum DisplayMode {
    Unknown = 0,
    Normal = 1,
    Minimal = 2,
    Off = 3
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct VehicleState {
    #[serde(rename = "_displayMode")]
    display_mode: Option<DisplayMode>,
    #[serde(rename = "_eventId")]
    event_id: Option<String>,
    #[serde(rename = "_eventType")]
    event_type: Option<String>,
    #[serde(rename = "_sequenceNumber")]
    sequnce_number: Option<i32>
}

impl VehicleState {

    pub fn display_mode(&self) -> &Option<DisplayMode> { &self.display_mode }
    
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_json_string_gpt() {
        let vehicle_state: Result<VehicleState, serde_json::Error> = serde_json::from_str(&GPT_DISPLAY_MODE_MSG);
        assert!(vehicle_state.is_ok());
    }
    #[test]
    fn verify_data_gpt() {
        let vehicle_state: Result<VehicleState, serde_json::Error> = serde_json::from_str(&GPT_DISPLAY_MODE_MSG);
        assert!(vehicle_state.is_ok());
        let vehicle_state = vehicle_state.unwrap();
        assert_eq!(vehicle_state.display_mode(), &Some(DisplayMode::Normal));
    }

    const GPT_DISPLAY_MODE_MSG: &str = "{\"_displayMode\":\"Normal\",\"_eventId\":\"5ef01976-156d-470c-92a6-2065d6981709\",\"_eventType\":\"mi/tdp/state\",\"_sequenceNumber\":11}";
}


