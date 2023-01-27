
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct DestinationDisplay {
    #[serde(rename = "number")]
    number: i32,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "alternativeText")]
    alternative_text: Option<String>,
    #[serde(rename = "lineDesignation")]
    line_designation: Option<String>,
    #[serde(rename = "typeOfAlternativeText")]
    type_of_alternative_text: Option<String>,

    #[serde(rename = "_eventId")]
    event_id: Option<String>,
    #[serde(rename = "_eventType")]
    event_type: Option<String>,
    #[serde(rename = "_sequenceNumber")]
    sequnce_number: Option<i32>
}

#[allow(dead_code)]
impl DestinationDisplay {
    // pub fn new (number: i32, name: String, alternative_text: Option<String>, line_designation: Option<String>, type_of_alternative_text: Option<String>, event_id: Option<String>, event_type: Option<String>, sequnce_number: Option<i32>) -> Self {
    //     DestinationDisplay {
    //         number,
    //         name,
    //         alternative_text,
    //         line_designation,
    //         type_of_alternative_text,
    //         event_id,
    //         event_type,
    //         sequnce_number
    //     }
    // }

    pub fn number(&self) -> &i32 { &self.number }

    pub fn name(&self) -> &str { &self.name.as_str() }

    pub fn alternative_text(&self) -> Option<&str> {
        if let Some(alternative_text_ret) = &self.alternative_text{
            return Some(&alternative_text_ret.as_str())
        }
        return None;
    }
    
    pub fn type_of_alternative_text(&self) -> Option<&str> {
        if let Some(type_of_alternative_text_ret) = &self.type_of_alternative_text{
            return Some(&type_of_alternative_text_ret.as_str())
        }
        return None;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn deserialize_json_string_gpt() {
        let destination_display: Result<DestinationDisplay, serde_json::Error> = serde_json::from_str(&GPT_MQTT_MSG);
        assert!(destination_display.is_ok());
    }
    #[test]
    fn verify_data_gpt() {
        let destination_display: Result<DestinationDisplay, serde_json::Error> = serde_json::from_str(&GPT_MQTT_MSG);
        assert!(destination_display.is_ok());

        let destination_display = destination_display.unwrap();
        let number = 0;
        assert_eq!(destination_display.number(), &number);

        let alternative_text: &str = "Ej i trafik";
        assert_eq!(destination_display.alternative_text(), Some(alternative_text));        
        assert_eq!(destination_display.type_of_alternative_text(), None);
        
    }
    
    #[test]
    fn deserialize_json_string_itxpt() {
        let destination_display: Result<DestinationDisplay, serde_json::Error> = serde_json::from_str(&ITXPT_EXAMPLE_MSG);
        assert!(destination_display.is_ok());

        let destination_display = destination_display.unwrap();

        let number = 98;
        assert_eq!(destination_display.number(), &number);
        
        let name = "Ej i trafik";
        assert_eq!(destination_display.name(), name);
        assert_eq!(destination_display.alternative_text(), None);        

    }


    const GPT_MQTT_MSG: &str = "{\"number\":0,\"name\":\"Snart linje 2\",\"lineDesignation\":\"\",\"alternativeText\":\"Ej i trafik\",\"_eventId\":\"b4b7ba37-b736-4b3b-82c6-39dc15f9a002\",\"_eventType\":\"oi/current_destination_display/text\",\"_sequenceNumber\":43}";
    const ITXPT_EXAMPLE_MSG: &str = "{\"number\": 98, \"name\": \"Ej i trafik\"}";
}

