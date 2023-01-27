
use chrono::{DateTime, NaiveDate, FixedOffset};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct BlockState {
    #[serde(rename = "fromDateTime")]
    from_date: DateTime<FixedOffset>,
    #[serde(rename = "blockRef")]
    block_ref: String,
    #[serde(rename = "state")]
    state: StateEnum,
    #[serde(rename = "source")]
    source: SourceEnum,
    #[serde(rename = "operatingDay")]
    operating_day: NaiveDate,

    #[serde(rename = "_eventId")]
    event_id: Option<String>,
    #[serde(rename = "_eventType")]
    event_type: Option<String>,
    #[serde(rename = "_sequenceNumber")]
    sequnce_number: Option<i32>
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum StateEnum {
    #[serde(rename = "ASSIGNED")]
    Assigned = 0,
    #[serde(rename = "SIGNED_ON")]
    SignedOn = 1,
    #[serde(rename = "NOT_ASSIGNED")]
    NotAssigned = 2
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum SourceEnum{
    BACKOFFICE = 0,
    LOCAL = 1
}

#[allow(dead_code)]
impl BlockState {
    // pub fn new (from_date: DateTime<FixedOffset>, block_ref: String, operating_day: NaiveDate, event_id: Option<String>, event_type: Option<String>, sequnce_number: Option<i32>) -> Self{
    //     BlockState {
    //         from_date,
    //         block_ref,
    //         operating_day,
    //         state: StateEnum::Assigned,
    //         source: SourceEnum::BACKOFFICE,
    //         event_id,
    //         event_type,
    //         sequnce_number
    //     }
    // }

    pub fn from_date(&self) -> &DateTime<FixedOffset>  { &self.from_date }

    pub fn operating_day(&self) -> &NaiveDate  { &self.operating_day }

    pub fn state(&self) -> &StateEnum  { &self.state }

    pub fn source(&self) -> &SourceEnum  { &self.source }

}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc, NaiveDate};
    
    #[test]
    fn deserialize_json_string_gpt() {
        let block_state: Result<BlockState, serde_json::Error> = serde_json::from_str(&GPT_MQTT_MSG);
        assert!(block_state.is_ok());
    }
    #[test]
    fn verify_data_gpt() {
        let block_state: Result<BlockState, serde_json::Error> = serde_json::from_str(&GPT_MQTT_MSG);
        assert!(block_state.is_ok());
        
        let block_state = block_state.unwrap();
        let from_date = DateTime::parse_from_rfc3339("2023-01-04T14:58:00.26Z").unwrap().with_timezone(&Utc);
        assert_eq!(block_state.clone().from_date(), &from_date);

        let operating_day = NaiveDate::parse_from_str("2023-01-04", "%Y-%m-%d").unwrap();
        assert_eq!(block_state.clone().operating_day(), &operating_day);

        let state = StateEnum::SignedOn;
        assert_eq!(block_state.clone().state(), &state);

        let source = SourceEnum::BACKOFFICE;
        assert_eq!(block_state.clone().source(), &source);
    }
    #[test]
    fn deserialize_json_string_itxpt() {
        let block_state: Result<BlockState, serde_json::Error> = serde_json::from_str(&ITXPT_EXAMPLE_MSG);
        assert!(block_state.is_ok());
    }
    #[test]
    fn verify_data_itxpt() {
        let block_state: Result<BlockState, serde_json::Error> = serde_json::from_str(&ITXPT_EXAMPLE_MSG);
        assert!(block_state.is_ok());
        
        let block_state = block_state.unwrap();
        let from_date = DateTime::parse_from_rfc3339("2017-10-31T12:45:50+01:00").unwrap().with_timezone(&Utc);
        assert_eq!(block_state.clone().from_date(), &from_date);

        let operating_day = NaiveDate::parse_from_str("2017-10-31", "%Y-%m-%d").unwrap();
        assert_eq!(block_state.clone().operating_day(), &operating_day);

        let state = StateEnum::SignedOn;
        assert_eq!(block_state.clone().state(), &state);

        let source = SourceEnum::BACKOFFICE;
        assert_eq!(block_state.clone().source(), &source);
    }

    const GPT_MQTT_MSG: &str = "{\"fromDateTime\":\"2023-01-04T14:58:00.26Z\",\"blockRef\":\"9041005900300626\",\"operatingDay\":\"2023-01-04\",\"state\":\"SIGNED_ON\",\"source\":\"BACKOFFICE\",\"_eventId\":\"af060e79-33c1-4827-be9f-66ce898ad4be\",\"_eventType\":\"oi/current_block/state\",\"_sequenceNumber\":26}";
    const ITXPT_EXAMPLE_MSG: &str = "{\"fromDateTime\": \"2017-10-31T12:45:50+01:00\",\"blockRef\": \"9041001002302341\",\"operatingDay\": \"2017-10-31\",\"state\": \"SIGNED_ON\",\"source\": \"BACKOFFICE\"}";
}



