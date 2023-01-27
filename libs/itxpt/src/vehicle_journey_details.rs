
use chrono::{DateTime, NaiveDate, FixedOffset};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum TransportMode
{
    #[serde(rename = "UNKNOWN")]
    Unknown = 0,
    #[serde(rename = "RAIL")]
    Rail = 100,
    #[serde(rename = "METRO")]
    Metro = 400,
    #[serde(rename = "BUS")]
    Bus = 700,
    #[serde(rename = "TRAM")]
    Tram = 900,
    #[serde(rename = "FERRY")]
    Ferry = 1000
}
#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum ArrivalType
{
    #[serde(rename = "UNKNOWN")]
    Unknown = 0,
    #[serde(rename = "NO_STOP")]
    NoStop = 1,
    #[serde(rename = "STOP_NO_ALIGHTING")]
    StopNoAlighting = 2,
    #[serde(rename = "STOP_IF_ALIGHTING")]
    StopIfAlighting = 3
}
#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum DepartureTypeEnum
{
    #[serde(rename = "UNKNOWN")]
    Unknown = 0,
    #[serde(rename = "NO_STOP")]
    NoStop = 1,
    #[serde(rename = "STOP_NO_BOARDING")]
    StopNoBoarding = 2,
    #[serde(rename = "STOP_IF_BOARDING")]
    StopIfBoarding = 3
}

mod naive_date_format {
    use chrono::{NaiveDate};
    use serde::{self, Deserialize, Serializer, Deserializer};
    use regex::Regex;


    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%z";
    const FORMAT2: &'static str = "%Y-%m-%dT%H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &NaiveDate,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        // Check if date contains timezone.
        let re = Regex::new(r"\+\d{2}:\d{2}$").unwrap();
        if re.is_match(&s) {
            NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
        }
        else {
            NaiveDate::parse_from_str(&s, FORMAT2).map_err(serde::de::Error::custom)

        }
    }

}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct VehicleJourneyDetails {
    #[serde(rename = "operatingDayDate", with="naive_date_format")]
    operating_day_date: NaiveDate,
    #[serde(rename = "vehicleJourneyRef")]
    vehicle_journey_ref: String,
    #[serde(rename = "journeyNumber")]
    journey_number: String,
    #[serde(rename = "journeyPatternRef")]
    #[deprecated(since = "0.1.0", note = "Field changed to work on GPT messages but is wrong according to ITxPT")]
    journey_pattern_ref: Option<String>,
    #[serde(rename = "timedJourneyPatternRef")]
    timed_journey_pattern_ref: Option<String>,
    #[serde(rename = "transportModeCode")]
    transport_mode_code: TransportMode,
    #[serde(rename = "transportAuthority")]
    transport_authority: OrganisationInfo,
    #[serde(rename = "contractor")]
    contractor: Option<OrganisationInfo>,
    #[serde(rename = "plannedStartDateTime")]
    planned_start_date_time: DateTime<FixedOffset>,
    #[serde(rename = "plannedEndDateTime")]
    planned_end_date_time: DateTime<FixedOffset>,
    #[serde(rename = "origin")]
    origin: Place,
    #[serde(rename = "line")]
    line: Option<LineInfo>,
    #[serde(rename = "directionOfLine")]
    direction_of_line: Option<DirectionOfLineInfo>,
    #[serde(rename = "calls")]
    calls: Vec<PointCall>,

    #[serde(rename = "gtfsShapeId")]
    gtfs_shape_id: Option<String>,
    #[serde(rename = "gtfsTripId")]
    gtfs_trip_id: Option<String>,
    #[serde(rename = "gptShapeHash")]
    gpt_shape_hash: Option<String>,
    #[serde(rename = "journeyPath")]
    journey_path: Option<Vec<Vec<f64>>>,
    #[serde(rename = "_eventId")]
    event_id: Option<String>,
    #[serde(rename = "_eventType")]
    event_type: Option<String>,
    #[serde(rename = "_sequenceNumber")]
    sequnce_number: Option<i32>
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct LineInfo {
    #[serde(rename = "ref")]
    ref_id: String,
    #[serde(rename = "designation")]
    designation: String,
    #[serde(rename = "number")]
    number: String,
    #[serde(rename = "name")]
    name: Option<String>
}
#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct DirectionOfLineInfo {
    code: String,
    name: Option<String>
}
#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Place {
    #[serde(rename = "name")]
    name: Option<String>,
    #[serde(rename = "shortName")]
    short_name: Option<String>,
}
#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct DestinationDisplay {
    #[serde(rename = "productName")]
    product_name: Option<String>,
    #[serde(rename = "symbolName")]
    symbol_name: Option<String>,
    #[serde(rename = "lineDesignation")]
    line_designation: String,
    #[serde(rename = "primaryDestination")]
    primary_destination: PrimaryDestination,
    #[serde(rename = "secondaryDestination")]
    secondary_destination: Option<SecondaryDestination>,
    #[serde(rename = "displayKeys")]
    display_keys: Option<Vec<Key>>
}
impl DestinationDisplay {
    pub fn line_designation(&self) -> &str { &self.line_designation.as_str() }

    pub fn primary_destination(&self) -> &str { &self.primary_destination.name() }

    pub fn secondary_destination(&self) -> Option<Vec<String>> {
        if let Some(secondary_destination_ret) = &self.secondary_destination{
            if let Some(secondary_destination_name) = secondary_destination_ret.name() {
                return Some(vec![secondary_destination_name.to_string()])
            }
        }
        return None;
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct PrimaryDestination {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "shortName")]
    short_name: Option<String>
}
impl PrimaryDestination {
    pub fn name (&self) -> &str { &self.name }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct SecondaryDestination {
    #[serde(rename = "name")]
    name: Option<String>,
    #[serde(rename = "shortName")]
    short_name: Option<String>,
    #[serde(rename = "secondaryDestinationType")]
    secondary_destination_type: Option<String>
}
impl SecondaryDestination {
    pub fn name (&self) -> Option<&str> {
        if let Some(name_ret) = &self.name {
            return Some(name_ret);
        }
        return None;
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct PointCall {
    #[serde(rename = "sequenceNumber")]
    sequence_number: u32,
    #[serde(rename = "journeyPatternPoint")]
    journey_pattern_point: JourneyPatternPointInfo,
    #[serde(rename = "stopArea")]
    stop_area: Option<StopAreaInfo>,
    #[serde(rename = "stopPoint")]
    stop_point: Option<StopPointInfo>,
    #[serde(rename = "arrival")]
    arrival: Arrival,
    #[serde(rename = "departure")]
    departure: Departure,
    #[serde(rename = "destinationDisplay")]
    destination_display: Option<DestinationDisplay>,
    #[serde(rename = "isCancelledCall")]
    is_cancelled_call: Option<bool>,
    #[serde(rename = "replaceJourneyPatternPoint")]
    replace_journey_pattern_point: Option<JourneyPatternPointInfo>,
    #[serde(rename = "replacedStopArea")]
    replaced_stop_area: Option<StopAreaInfo>,
    #[serde(rename = "replacedStopPoint")]
    replaced_stop_point: Option<StopPointInfo>,
    #[serde(rename = "detourEnroute")]
    detour_enroute: Option<DetourEnroute>,
    #[serde(rename = "fetcherConnections")]
    fetcher_connections: Option<Vec<ConnectionInfo>>,
    #[serde(rename = "feederConnections")]
    feeder_connections: Option<Vec<ConnectionInfo>>
}
impl PointCall {
    pub fn stop_point(&self) -> &Option<StopPointInfo> { &self.stop_point }
    
    pub fn sequence_number(&self) -> u32 { self.sequence_number }
    
    pub fn arrival(&self) -> &Arrival { &self.arrival }

    pub fn destination_display(&self) -> &Option<DestinationDisplay> { &self.destination_display }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct JourneyPatternPointInfo {
    #[serde(rename = "ref")]
    #[deprecated(since = "0.1.0", note = "Field changed to work on GPT messages but is wrong according to ITxPT")]
    ref_id: Option<String>,
    #[serde(rename = "isTimingPoint")]
    is_timing_point: bool,
    #[serde(rename = "location")]
    location: Option<Position>,
    #[serde(rename = "distanceFromPrevious")]
    distance_from_previous: Option<f64>,
    #[serde(rename = "detection")]
    detection: Option<Detection>,
    #[serde(rename = "tariffZones")]
    tariff_zones: Option<Vec<TariffZoneInfo>>,
    #[serde(rename = "pathFromPrevious")]
    path_from_previous: Option<Path>
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Position {
    latitude: f64,
    longitude: f64
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Detection {
    #[serde(rename = "enteringDistance")]
    entering_distance: Option<u32>,
    #[serde(rename = "exitingDistance")]
    exiting_distance: Option<u32>,
    #[serde(rename = "passingDirection")]
    passing_direction: Option<PassingDirection>
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct PassingDirection {
    #[serde(rename = "direction")]
    direction: Option<f64>,
    #[serde(rename = "maxDeviation")]
    max_deviation: Option<f64>
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Path {
    #[serde(rename = "coordinates")]
    coordinates: Vec<Vec<f64>>
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct DetourEnroute {
    #[serde(rename = "startsAfterCallSequenceNumber")]
    starts_after_call_sequence_number: u32,
    #[serde(rename = "instructions")]
    instructions: String,
    #[serde(rename = "path")]
    path: Path
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct StopAreaInfo {
    #[serde(rename = "ref")]
    ref_id: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "shortName")]
    short_name: Option<String>
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct StopPointInfo {
    #[serde(rename = "ref")]
    ref_id: String,
    #[serde(rename = "name")]
    name: Option<String>,
    #[serde(rename = "shortName")]
    short_name: Option<String>,
    #[serde(rename = "designation")]
    designation: Option<String>,
    #[serde(rename = "localNumber")]
    #[deprecated(since = "0.1.0", note = "Field changed to work on GPT messages but is wrong according to ITxPT")]
    local_number: Option<u32>,
    #[serde(rename = "length")]
    length: Option<f64>,
    #[serde(rename = "orientation")]
    orientation: Option<f64>,
    #[serde(rename = "stopPointKeys")]
    stop_point_keys: Option<Vec<Key>>
}
impl StopPointInfo {
    pub fn name(&self) -> Option<&str> {
        if let Some(name_ret) = &self.name{
            return Some(&name_ret.as_str())
        }
        return None;
    }

    pub fn ref_id(&self) -> &str { &self.ref_id.as_str() }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Arrival {
    #[serde(rename = "latestDateTime")]
    latest_date_time: DateTime<FixedOffset>,
    #[serde(rename = "arrivalType")]
    arrival_type: ArrivalType
}
impl Arrival {
    pub fn arrival_type(&self) -> &ArrivalType { &self.arrival_type }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Departure {
    #[serde(rename = "earliestDateTime")]
    earliest_date_time: DateTime<FixedOffset>,
    #[serde(rename = "departureType")]
    departure_type: DepartureTypeEnum
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct OrganisationInfo {
    #[serde(rename = "ref")]
    ref_id: String,
    #[serde(rename = "code")]
    code: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "number")]
    number: String
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Key {
    #[serde(rename = "deviceName")]
    device_name: String,
    #[serde(rename = "typeCode")]
    type_code: String,
    #[serde(rename = "parameterData")]
    parameter_data: String    
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct TariffZoneInfo {
    #[serde(rename = "ref")]
    ref_id: String,
    #[serde(rename = "number")]
    number: u32,
    #[serde(rename = "code")]
    code: Option<String>,
    #[serde(rename = "name")]
    name: Option<String>
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct ConnectionInfo {
    #[serde(rename = "connectionRef")]
    connection_ref: String,
    #[serde(rename = "transportModeCode")]
    transport_mode_code: TransportMode,
    #[serde(rename = "lineAuthorityCode")]
    line_authority_code: String,
    #[serde(rename = "lineDesignation")]
    line_designation: String,
    #[serde(rename = "directionName")]
    direction_name: Option<String>,
    #[serde(rename = "stopAreaName")]
    stop_area_name: Option<String>,
    #[serde(rename = "stopPointDesignation")]
    stop_point_designation: Option<String>,
    #[serde(rename = "minChangeDurationSeconds")]
    min_change_duration_seconds: u32,
    #[serde(rename = "maxWaitingUntilTime")]
    max_waiting_until_time: Option<DateTime<FixedOffset>>
}

#[allow(dead_code)]
impl VehicleJourneyDetails {
    // pub fn new (operating_day_date: NaiveDate, vehicle_journey_ref: String, journey_number: String, journey_pattern_ref: Option<String>, timed_journey_pattern_ref: Option<String>, transport_mode_code: TransportMode,
    //     transport_authority: OrganisationInfo, contractor: Option<OrganisationInfo>, planned_start_date_time: DateTime<FixedOffset>, planned_end_date_time: DateTime<FixedOffset>, origin: Place, line: Option<LineInfo>, 
    //     direction_of_line: Option<DirectionOfLineInfo>, calls: Vec<PointCall>, gtfs_shape_id: Option<String>, gtfs_trip_id: Option<String>, gpt_shape_hash: Option<String>, journey_path: Option<Vec<Vec<f64>>>,
    //     event_id: Option<String>, event_type: Option<String>, sequnce_number: Option<i32>) -> Self{
    //     VehicleJourneyDetails {
    //         operating_day_date,
    //         vehicle_journey_ref,
    //         journey_number,
    //         journey_pattern_ref,
    //         timed_journey_pattern_ref,
    //         transport_mode_code,
    //         transport_authority,
    //         contractor,
    //         planned_start_date_time,
    //         planned_end_date_time,
    //         origin,
    //         line,
    //         direction_of_line,
    //         calls,
    //         gtfs_shape_id,
    //         gtfs_trip_id,
    //         gpt_shape_hash,
    //         journey_path,
    //         event_id,
    //         event_type,
    //         sequnce_number
    //     }
    // }

    pub fn operating_day_date(&self) -> &NaiveDate { &self.operating_day_date }

    pub fn origin(&self) -> &Place { &self.origin }
    
    pub fn planned_start_date_time(&self) -> &DateTime<FixedOffset> { &self.planned_start_date_time }
    
    pub fn planned_end_date_time(&self) -> &DateTime<FixedOffset> { &self.planned_end_date_time }

    pub fn transport_mode_code(&self) -> &TransportMode { &self.transport_mode_code }

    pub fn line(&self) -> &Option<LineInfo> { &self.line }

    pub fn direction_of_line(&self) -> &Option<DirectionOfLineInfo> { &self.direction_of_line }

    pub fn transport_authority(&self) -> &OrganisationInfo { &self.transport_authority }

    pub fn contractor(&self) -> &Option<OrganisationInfo> { &self.contractor }

    pub fn calls(&self) -> &Vec<PointCall> { &self.calls }

    pub fn journey_path(&self) -> &Option<Vec<Vec<f64>>> {
        return &self.journey_path;
    }

    pub fn gtfs_shape_id(&self) -> Option<&str> {
        if let Some(gtfs_shape_id_ret) = &self.gtfs_shape_id{
            return Some(&gtfs_shape_id_ret.as_str())
        }
        return None;
    }

    pub fn gtfs_trip_id(&self) -> Option<&str> {
        if let Some(gtfs_trip_id_ret) = &self.gtfs_trip_id{
            return Some(&gtfs_trip_id_ret.as_str())
        }
        return None;
    }

    pub fn gpt_shape_hash(&self) -> Option<&str> {
        if let Some(gpt_shape_hash_ret) = &self.gpt_shape_hash{
            return Some(&gpt_shape_hash_ret.as_str())
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




#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, NaiveDate};
    
    #[test]
    fn deserialize_json_string_gpt() {
        let vehicle_journey_details: Result<VehicleJourneyDetails, serde_json::Error> = serde_json::from_str(&GPT_MQTT_MSG);
        assert!(vehicle_journey_details.is_ok());
    }
    #[test]
    fn verify_data_gpt() {
        let vehicle_journey_details: Result<VehicleJourneyDetails, serde_json::Error> = serde_json::from_str(&GPT_MQTT_MSG);
        assert!(vehicle_journey_details.is_ok());

        let vehicle_journey_details = vehicle_journey_details.unwrap();

        assert_eq!(vehicle_journey_details.gtfs_shape_id(), Some("2055700000054710291"));
        assert_eq!(vehicle_journey_details.gtfs_trip_id(), Some("55700000066530291"));
        assert_eq!(vehicle_journey_details.gpt_shape_hash(), Some("336DB33F6783B37540A78C70CA4165E1"));
        assert_eq!(vehicle_journey_details.event_id(), Some("4868ad61-5edb-4e0b-b827-fac49b6ee2a0"));
        assert_eq!(vehicle_journey_details.event_type(), Some("oi/current_vehicle_journey/details"));
        assert_eq!(vehicle_journey_details.sequnce_number(), Some(&6));

        let vec_journey_path_length = 173;
        let first_elem = Some(vec![58.60855865478515, 16.151491165161133]);
        if let Some(vec) = vehicle_journey_details.journey_path() {
            assert_eq!(vec.len(), vec_journey_path_length);
            assert_eq!(Some(vec[0].clone()), first_elem);
        }


        let operating_day_date = NaiveDate::parse_from_str("2023-01-12T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        assert_eq!(vehicle_journey_details.operating_day_date(), &operating_day_date);

        let transport_mode_code = TransportMode::Tram;
        assert_eq!(vehicle_journey_details.transport_mode_code(), &transport_mode_code);

        let line = Some(LineInfo {
            ref_id: "9011005000300000".to_string(),
            designation: "3".to_string(),
            number: "9011005000300000".to_string(),
            name: None
        });
        assert_eq!(vehicle_journey_details.line(), &line);

        let direction_of_line = Some(DirectionOfLineInfo {
            code: "1".to_string(),
            name: Some("".to_string())
        });
        assert_eq!(vehicle_journey_details.direction_of_line(), &direction_of_line);

        let transport_authority = OrganisationInfo {
            ref_id: "9010005000000000".to_string(),
            code: "OTN".to_string(),
            name: "Östgötatrafiken".to_string(),
            number: "5".to_string()
        };
        assert_eq!(vehicle_journey_details.transport_authority(), &transport_authority);

        let contractor = None;
        assert_eq!(vehicle_journey_details.contractor(), &contractor);

        let planned_start_date_time = DateTime::parse_from_rfc3339("2023-01-12T09:14:00Z").unwrap();
        assert_eq!(vehicle_journey_details.planned_start_date_time(), &planned_start_date_time);
        let planned_end_date_time = DateTime::parse_from_rfc3339("2023-01-12T09:49:00Z").unwrap();
        assert_eq!(vehicle_journey_details.planned_end_date_time(), &planned_end_date_time);

        let origin = Place {
            name: Some("Vidablick".to_string()),
            short_name: None
        };
        assert_eq!(vehicle_journey_details.origin(), &origin);

        let vec_calls_length = 22;
        let vec = vehicle_journey_details.calls();
        assert_eq!(vec.len(), vec_calls_length);

        let point_call = PointCall {
            sequence_number: 1,
            journey_pattern_point: JourneyPatternPointInfo {
                ref_id: None,
                is_timing_point: true,
                location: None,
                distance_from_previous: None,
                detection: None,
                path_from_previous: None,
                tariff_zones: None
            },
            stop_area: Some(StopAreaInfo {
                ref_id: "9021005000012000".to_string(),
                name: "Vidablick".to_string(),
                short_name: None
            }),
            stop_point: Some(StopPointInfo {
                ref_id: "9022005000012016".to_string(),
                name: Some("Vidablick".to_string()),
                short_name: None,
                designation: Some("A".to_string()),
                local_number: None,
                length: None,
                orientation: None,
                stop_point_keys: None
            }),
            arrival: Arrival {
                latest_date_time: DateTime::parse_from_rfc3339("2023-01-12T09:14:00Z").unwrap(),
                arrival_type: ArrivalType::StopNoAlighting
            },
            departure: Departure {
                earliest_date_time: DateTime::parse_from_rfc3339("2023-01-12T09:14:00Z").unwrap(),
                departure_type: DepartureTypeEnum::StopIfBoarding
            },
            destination_display: Some(DestinationDisplay {
                product_name: None,
                symbol_name: None,
                line_designation: "3".to_string(),
                primary_destination: PrimaryDestination {
                    name: "Klockaretorpet".to_string(),
                    short_name: None
                },
                secondary_destination: None,
                display_keys: None
            }),
            detour_enroute: None,
            feeder_connections: None,
            fetcher_connections: None,
            is_cancelled_call: None,
            replace_journey_pattern_point: None,
            replaced_stop_area: None,
            replaced_stop_point: None
        }; 
        assert_eq!(vehicle_journey_details.calls()[0], point_call);
    }

    #[test]
    fn deserialize_json_string_itxpt() {
        let vehicle_journey_details: Result<VehicleJourneyDetails, serde_json::Error> = serde_json::from_str(&ITXPT_EXAMPLE_MSG);
        assert!(vehicle_journey_details.is_ok());
    }
    #[test]
    fn verify_data_itxpt() {
        let vehicle_journey_details: Result<VehicleJourneyDetails, serde_json::Error> = serde_json::from_str(&ITXPT_EXAMPLE_MSG);
        assert!(vehicle_journey_details.is_ok());

        let vehicle_journey_details = vehicle_journey_details.unwrap();

        let operating_day_date = NaiveDate::parse_from_str("2017-04-27T00:00:00+02:00", "%Y-%m-%dT%H:%M:%S%z").unwrap();
        assert_eq!(vehicle_journey_details.operating_day_date(), &operating_day_date);

        let transport_mode_code = TransportMode::Bus;
        assert_eq!(vehicle_journey_details.transport_mode_code(), &transport_mode_code);

        let line = Some(LineInfo {
            ref_id: "9011001095000000".to_string(),
            designation: "22".to_string(),
            number: "950".to_string(),
            name: Some("Ersätter Tvärbanan".to_string())
        });
        assert_eq!(vehicle_journey_details.line(), &line);

        let direction_of_line = Some(DirectionOfLineInfo {
            code: "1".to_string(),
            name: Some("Solna".to_string())
        });
        assert_eq!(vehicle_journey_details.direction_of_line(), &direction_of_line);

        let transport_authority = OrganisationInfo {
            ref_id: "9010001000000000".to_string(),
            code: "SL".to_string(),
            name: "Storstockholms Lokaltrafik".to_string(),
            number: "1".to_string()
        };
        assert_eq!(vehicle_journey_details.transport_authority(), &transport_authority);

        let contractor = Some(OrganisationInfo {
            ref_id: "9013001001500000".to_string(),
            code: "ARR".to_string(),
            name: "Arriva".to_string(),
            number: "15".to_string()
        });
        assert_eq!(vehicle_journey_details.contractor(), &contractor);

        let planned_start_date_time = DateTime::parse_from_rfc3339("2017-04-27T05:51:00+02:00").unwrap();
        assert_eq!(vehicle_journey_details.planned_start_date_time(), &planned_start_date_time);
        let planned_end_date_time = DateTime::parse_from_rfc3339("2017-04-27T06:39:00+02:00").unwrap();
        assert_eq!(vehicle_journey_details.planned_end_date_time(), &planned_end_date_time);

        let origin = Place {
            name: Some("Sickla udde".to_string()),
            short_name: Some("Sickla udde".to_string())
        };
        assert_eq!(vehicle_journey_details.origin(), &origin);

        let vec_calls_length = 2;
        let vec = vehicle_journey_details.calls();
        assert_eq!(vec.len(), vec_calls_length);

        let point_call = PointCall {
            sequence_number: 1,
            journey_pattern_point: JourneyPatternPointInfo {
                ref_id: Some("9025001000010664".to_string()),
                is_timing_point: true,
                location: Some(Position {
                    latitude: 59.3071932996622,
                    longitude: 18.1078200699196
                }),
                distance_from_previous: Some(0.0),
                detection: Some(Detection {
                    entering_distance: Some(20),
                    exiting_distance: Some(10),
                    passing_direction: None
                }),
                path_from_previous: None,
                tariff_zones: None
            },
            stop_area: Some(StopAreaInfo {
                ref_id: "9021001010665000".to_string(),
                name: "Sickla udde".to_string(),
                short_name: Some("Sickla udde".to_string()),
            }),
            stop_point: Some(StopPointInfo {
                ref_id: "9022001010665005".to_string(),
                name: Some("Sickla udde".to_string()),
                short_name: Some("Sickla udde".to_string()),
                designation: Some("".to_string()),
                local_number: Some(5),
                length: None,
                orientation: None,
                stop_point_keys: None
            }),
            arrival: Arrival {
                latest_date_time: DateTime::parse_from_rfc3339("2017-04-27T05:51:00+02:00").unwrap(),
                arrival_type: ArrivalType::StopNoAlighting
            },
            departure: Departure {
                earliest_date_time: DateTime::parse_from_rfc3339("2017-04-27T05:51:00+02:00").unwrap(),
                departure_type: DepartureTypeEnum::StopIfBoarding
            },
            destination_display: Some(DestinationDisplay {
                product_name: Some("".to_string()),
                symbol_name: Some("".to_string()),
                line_designation: "22B".to_string(),
                primary_destination: PrimaryDestination {
                    name: "Thorildsplan".to_string(),
                    short_name: Some("".to_string())
                },
                secondary_destination: Some(SecondaryDestination {
                    name: Some("Eriksbo värdshuset".to_string()),
                    short_name: Some("Eriksbo".to_string()),
                    secondary_destination_type: Some("VIA".to_string())
                }),
                display_keys: Some(vec![Key {
                    parameter_data: "Destination=653".to_string(),
                    type_code: "O_DESTIN".to_string(),
                    device_name: "SIGN".to_string()
                }])
            }),
            detour_enroute: None,
            feeder_connections: None,
            fetcher_connections: None,
            is_cancelled_call: None,
            replace_journey_pattern_point: None,
            replaced_stop_area: None,
            replaced_stop_point: None
        }; 
        assert_eq!(vehicle_journey_details.calls()[0], point_call);
    }
    
    const GPT_MQTT_MSG: &str = "{\"operatingDayDate\":\"2023-01-12T00:00:00\",\"vehicleJourneyRef\":\"9015005000300065\",\"journeyNumber\":\"65\",\"transportModeCode\":\"TRAM\",\"transportAuthority\":{\"ref\":\"9010005000000000\",\"code\":\"OTN\",\"name\":\"Östgötatrafiken\",\"number\":\"5\"},\"plannedStartDateTime\":\"2023-01-12T09:14:00Z\",\"plannedEndDateTime\":\"2023-01-12T09:49:00Z\",\"origin\":{\"name\":\"Vidablick\"},\"line\":{\"ref\":\"9011005000300000\",\"designation\":\"3\",\"number\":\"9011005000300000\"},\"directionOfLine\":{\"code\":\"1\",\"name\":\"\"},\"calls\":[{\"sequenceNumber\":1,\"journeyPatternPoint\":{\"isTimingPoint\":true},\"stopArea\":{\"ref\":\"9021005000012000\",\"name\":\"Vidablick\"},\"stopPoint\":{\"ref\":\"9022005000012016\",\"name\":\"Vidablick\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:14:00Z\",\"arrivalType\":\"STOP_NO_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:14:00Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":2,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001039000\",\"name\":\"Rågången\"},\"stopPoint\":{\"ref\":\"9022005001039016\",\"name\":\"Rågången\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:14:56Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:14:56Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":3,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001038000\",\"name\":\"Sandbyhov\"},\"stopPoint\":{\"ref\":\"9022005001038016\",\"name\":\"Sandbyhov\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:16:10Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:16:10Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":4,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001037000\",\"name\":\"Breda vägen\"},\"stopPoint\":{\"ref\":\"9022005001037016\",\"name\":\"Breda vägen\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:17:36Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:17:36Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":5,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005000006000\",\"name\":\"Hagaskolan\"},\"stopPoint\":{\"ref\":\"9022005000006016\",\"name\":\"Hagaskolan\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:19:12Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:19:12Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":6,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001020000\",\"name\":\"Marielund\"},\"stopPoint\":{\"ref\":\"9022005001020016\",\"name\":\"Marielund\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:20:36Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:20:36Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":7,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001002000\",\"name\":\"Matteusskolan\"},\"stopPoint\":{\"ref\":\"9022005001002016\",\"name\":\"Matteusskolan\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:22:01Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:22:01Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":8,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005000001000\",\"name\":\"Norr Tull\"},\"stopPoint\":{\"ref\":\"9022005000001016\",\"name\":\"Norr Tull\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:23:36Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:23:36Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":9,\"journeyPatternPoint\":{\"isTimingPoint\":true},\"stopArea\":{\"ref\":\"9021005001000000\",\"name\":\"Norrköpings resecentrum\"},\"stopPoint\":{\"ref\":\"9022005001000016\",\"name\":\"Norrköpings resecentrum\",\"designation\":\"D1\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:26:00Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:26:00Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":10,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005000005000\",\"name\":\"Rådhuset\"},\"stopPoint\":{\"ref\":\"9022005000005016\",\"name\":\"Rådhuset\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:28:29Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:28:29Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":11,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005000011000\",\"name\":\"Hörsalsparken\"},\"stopPoint\":{\"ref\":\"9022005000011016\",\"name\":\"Hörsalsparken\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:31:00Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:31:00Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":12,\"journeyPatternPoint\":{\"isTimingPoint\":true},\"stopArea\":{\"ref\":\"9021005000003000\",\"name\":\"Söder Tull\"},\"stopPoint\":{\"ref\":\"9022005000003018\",\"name\":\"Söder Tull\",\"designation\":\"H\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:33:00Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:34:00Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":13,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005000008000\",\"name\":\"Väster Tull\"},\"stopPoint\":{\"ref\":\"9022005000008016\",\"name\":\"Väster Tull\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:36:26Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:36:26Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":14,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001027000\",\"name\":\"Strömbacken\"},\"stopPoint\":{\"ref\":\"9022005001027016\",\"name\":\"Strömbacken\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:37:42Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:37:42Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":15,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001029000\",\"name\":\"Vägträffen\"},\"stopPoint\":{\"ref\":\"9022005001029016\",\"name\":\"Vägträffen\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:39:00Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:39:00Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":16,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001030000\",\"name\":\"Lokegatan\"},\"stopPoint\":{\"ref\":\"9022005001030016\",\"name\":\"Lokegatan\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:40:36Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:40:36Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":17,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001031000\",\"name\":\"Skarphagsgatan\"},\"stopPoint\":{\"ref\":\"9022005001031016\",\"name\":\"Skarphagsgatan\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:41:29Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:41:29Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":18,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005000009000\",\"name\":\"Folkets Park\"},\"stopPoint\":{\"ref\":\"9022005000009016\",\"name\":\"Folkets Park\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:42:36Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:42:36Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":19,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001032000\",\"name\":\"SMHI\"},\"stopPoint\":{\"ref\":\"9022005001032016\",\"name\":\"SMHI\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:43:34Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:43:34Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":20,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001033000\",\"name\":\"Bastuban\"},\"stopPoint\":{\"ref\":\"9022005001033016\",\"name\":\"Bastuban\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:45:01Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:45:01Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":21,\"journeyPatternPoint\":{\"isTimingPoint\":false},\"stopArea\":{\"ref\":\"9021005001034000\",\"name\":\"Klockaretorpets centrum\"},\"stopPoint\":{\"ref\":\"9022005001034016\",\"name\":\"Klockaretorpets centrum\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:46:09Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:46:09Z\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}},{\"sequenceNumber\":22,\"journeyPatternPoint\":{\"isTimingPoint\":true},\"stopArea\":{\"ref\":\"9021005000010000\",\"name\":\"Klockaretorpets vändplats\"},\"stopPoint\":{\"ref\":\"9022005000010016\",\"name\":\"Klockaretorpets vändplats\",\"designation\":\"A\"},\"arrival\":{\"latestDateTime\":\"2023-01-12T09:49:00Z\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2023-01-12T09:49:00Z\",\"departureType\":\"STOP_NO_BOARDING\"},\"destinationDisplay\":{\"lineDesignation\":\"3\",\"primaryDestination\":{\"name\":\"Klockaretorpet\"}}}],\"gtfsShapeId\":\"2055700000054710291\",\"gtfsTripId\":\"55700000066530291\",\"gptShapeHash\":\"336DB33F6783B37540A78C70CA4165E1\",\"journeyPath\":[[58.608558654785156,16.151491165161133],[58.60854721069336,16.151491165161133],[58.608089447021484,16.152175903320312],[58.607383728027344,16.153200149536133],[58.60657501220703,16.154396057128906],[58.604881286621094,16.156909942626953],[58.60476303100586,16.15709686279297],[58.6047248840332,16.157182693481445],[58.604652404785156,16.15738868713379],[58.604610443115234,16.157577514648438],[58.60457229614258,16.15778350830078],[58.60456085205078,16.158109664916992],[58.60459518432617,16.158935546875],[58.60472869873047,16.162206649780273],[58.604740142822266,16.16251564025879],[58.60472869873047,16.162687301635742],[58.60470962524414,16.162790298461914],[58.60464859008789,16.163047790527344],[58.6046028137207,16.16316795349121],[58.60453796386719,16.163270950317383],[58.60445022583008,16.163389205932617],[58.604339599609375,16.163475036621094],[58.604225158691406,16.163524627685547],[58.60354995727539,16.16351890563965],[58.60343551635742,16.16349983215332],[58.60334396362305,16.16346549987793],[58.60310363769531,16.163358688354492],[58.60293960571289,16.163288116455078],[58.602813720703125,16.163236618041992],[58.60189437866211,16.162487030029297],[58.600746154785156,16.161357879638672],[58.600379943847656,16.160974502563477],[58.600101470947266,16.160680770874023],[58.59996795654297,16.160541534423828],[58.59990310668945,16.16048812866211],[58.59983444213867,16.160470962524414],[58.59977722167969,16.160503387451172],[58.599708557128906,16.16058921813965],[58.59964370727539,16.16072654724121],[58.59943771362305,16.161394119262695],[58.59942626953125,16.16143035888672],[58.599327087402344,16.16179084777832],[58.59829330444336,16.165029525756836],[58.59823226928711,16.16520118713379],[58.59819412231445,16.165287017822266],[58.598114013671875,16.1654052734375],[58.5980339050293,16.165525436401367],[58.597599029541016,16.166072845458984],[58.59745788574219,16.16619110107422],[58.596961975097656,16.166444778442383],[58.59406661987305,16.167943954467773],[58.59394073486328,16.168046951293945],[58.59388732910156,16.168115615844727],[58.593849182128906,16.16819953918457],[58.59382247924805,16.168371200561523],[58.593814849853516,16.16849136352539],[58.59382247924805,16.168577194213867],[58.593929290771484,16.169164657592773],[58.59400939941406,16.169645309448242],[58.594749450683594,16.174882888793945],[58.594905853271484,16.1760196685791],[58.595054626464844,16.177017211914062],[58.59553909301758,16.18042755126953],[58.5955924987793,16.1806697845459],[58.59577178955078,16.181238174438477],[58.5958251953125,16.181446075439453],[58.59602737426758,16.182857513427734],[58.59609603881836,16.183391571044922],[58.59612274169922,16.183616638183594],[58.59612274169922,16.18370246887207],[58.59610366821289,16.183839797973633],[58.5960693359375,16.183975219726562],[58.596031188964844,16.184045791625977],[58.595985412597656,16.18409538269043],[58.595943450927734,16.18412971496582],[58.59571838378906,16.18426513671875],[58.59444046020508,16.184904098510742],[58.59294509887695,16.185834884643555],[58.59234619140625,16.186153411865234],[58.591732025146484,16.18645668029785],[58.59132766723633,16.186641693115234],[58.587928771972656,16.18837547302246],[58.587894439697266,16.188392639160156],[58.587623596191406,16.188562393188477],[58.5862922668457,16.189252853393555],[58.58606719970703,16.1893367767334],[58.585655212402344,16.18955421447754],[58.58552932739258,16.189605712890625],[58.58546447753906,16.189638137817383],[58.58542251586914,16.189638137817383],[58.58536911010742,16.189603805541992],[58.585330963134766,16.189550399780273],[58.585304260253906,16.189481735229492],[58.585289001464844,16.189395904541016],[58.58527755737305,16.189292907714844],[58.585289001464844,16.18915557861328],[58.58533477783203,16.18879508972168],[58.58538818359375,16.188159942626953],[58.585975646972656,16.181289672851562],[58.5860595703125,16.180137634277344],[58.58634948730469,16.176788330078125],[58.586448669433594,16.17560386657715],[58.586544036865234,16.17467498779297],[58.58678436279297,16.17184066772461],[58.58678436279297,16.17154884338379],[58.58675765991211,16.171255111694336],[58.5865364074707,16.169878005981445],[58.586280822753906,16.16829490661621],[58.586124420166016,16.16750144958496],[58.58587646484375,16.16641616821289],[58.5851936340332,16.163177490234375],[58.5851936340332,16.16314125061035],[58.58512878417969,16.162918090820312],[58.585060119628906,16.162744522094727],[58.5849609375,16.16259002685547],[58.584815979003906,16.162450790405273],[58.58161163330078,16.159770965576172],[58.581539154052734,16.159683227539062],[58.581478118896484,16.15958023071289],[58.581424713134766,16.159408569335938],[58.58126449584961,16.15880584716797],[58.581199645996094,16.1585636138916],[58.581058502197266,16.15814971923828],[58.58064651489258,16.157182693481445],[58.58018493652344,16.156164169311523],[58.579872131347656,16.155420303344727],[58.57966613769531,16.15488624572754],[58.579559326171875,16.15460968017578],[58.57846450805664,16.15200424194336],[58.578250885009766,16.151416778564453],[58.57809829711914,16.150968551635742],[58.578033447265625,16.150728225708008],[58.5779914855957,16.15045166015625],[58.57798385620117,16.1502628326416],[58.57797622680664,16.149730682373047],[58.57797622680664,16.149368286132812],[58.57798767089844,16.14916229248047],[58.57802200317383,16.149009704589844],[58.57807540893555,16.148906707763672],[58.578147888183594,16.1488037109375],[58.57863235473633,16.148258209228516],[58.579524993896484,16.147253036499023],[58.57965087890625,16.147064208984375],[58.579689025878906,16.1469783782959],[58.579715728759766,16.146894454956055],[58.57975387573242,16.14668846130371],[58.57975387573242,16.146516799926758],[58.579742431640625,16.14630889892578],[58.579654693603516,16.145896911621094],[58.57883834838867,16.141830444335938],[58.578460693359375,16.140005111694336],[58.57832717895508,16.13931655883789],[58.57828140258789,16.138973236083984],[58.57822036743164,16.138404846191406],[58.57815170288086,16.137664794921875],[58.57809829711914,16.13726806640625],[58.57746124267578,16.133670806884766],[58.577335357666016,16.1330509185791],[58.57716751098633,16.13234519958496],[58.57670593261719,16.130329132080078],[58.57650375366211,16.12938117980957],[58.57646560668945,16.12908935546875],[58.57645797729492,16.12828254699707],[58.57643127441406,16.128005981445312],[58.57639694213867,16.127782821655273],[58.57632827758789,16.127506256103516],[58.576255798339844,16.12726593017578],[58.5761833190918,16.127092361450195],[58.576114654541016,16.126937866210938],[58.575862884521484,16.1264705657959],[58.57561111450195,16.126022338867188],[58.57548522949219,16.12581443786621],[58.575504302978516,16.125797271728516]],\"_eventId\":\"4868ad61-5edb-4e0b-b827-fac49b6ee2a0\",\"_eventType\":\"oi/current_vehicle_journey/details\",\"_sequenceNumber\":6}";
    const ITXPT_EXAMPLE_MSG: &str = "{\"operatingDayDate\":\"2017-04-27T00:00:00+02:00\",\"vehicleJourneyRef\":\"9015001095045020\",\"journeyNumber\":\"45020\",\"journeyPatternRef\":\"4010000482522803\",\"timedJourneyPatternRef\":\"4010000482522978\",\"transportModeCode\":\"BUS\",\"line\":{\"ref\":\"9011001095000000\",\"designation\":\"22\",\"number\":\"950\",\"name\":\"Ersätter Tvärbanan\"},\"directionOfLine\":{\"code\":\"1\",\"name\":\"Solna\"},\"transportAuthority\":{\"ref\":\"9010001000000000\",\"code\":\"SL\",\"name\":\"Storstockholms Lokaltrafik\",\"number\":\"1\"},\"contractor\":{\"ref\":\"9013001001500000\",\"code\":\"ARR\",\"name\":\"Arriva\",\"number\":\"15\"},\"plannedStartDateTime\":\"2017-04-27T05:51:00+02:00\",\"plannedEndDateTime\":\"2017-04-27T06:39:00+02:00\",\"origin\":{\"name\":\"Sickla udde\",\"shortName\":\"Sickla udde\"},\"calls\":[{\"sequenceNumber\":1,\"journeyPatternPoint\":{\"ref\":\"9025001000010664\",\"isTimingPoint\":true,\"location\":{\"latitude\":59.3071932996622,\"longitude\":18.1078200699196},\"distanceFromPrevious\":0,\"detection\":{\"enteringDistance\":20,\"exitingDistance\":10}},\"stopArea\":{\"ref\":\"9021001010665000\",\"name\":\"Sickla udde\",\"shortName\":\"Sickla udde\"},\"stopPoint\":{\"ref\":\"9022001010665005\",\"name\":\"Sickla udde\",\"shortName\":\"Sickla udde\",\"designation\":\"\",\"localNumber\":5},\"arrival\":{\"latestDateTime\":\"2017-04-27T05:51:00+02:00\",\"arrivalType\":\"STOP_NO_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2017-04-27T05:51:00+02:00\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"productName\":\"\",\"symbolName\":\"\",\"lineDesignation\":\"22B\",\"primaryDestination\":{\"name\":\"Thorildsplan\",\"shortName\":\"\"},\"secondaryDestination\":{\"name\":\"Eriksbo värdshuset\",\"shortName\":\"Eriksbo\",\"secondaryDestinationType\":\"VIA\"},\"displayKeys\":[{\"parameterData\":\"Destination=653\",\"typeCode\":\"O_DESTIN\",\"deviceName\":\"SIGN\"}]}},{\"sequenceNumber\":2,\"journeyPatternPoint\":{\"ref\":\"9025001000010667\",\"isTimingPoint\":true,\"location\":{\"latitude\":59.97882,\"longitude\":18.21584},\"distanceFromPrevious\":652,\"detection\":{\"enteringDistance\":20,\"exitingDistance\":10},\"pathFromPrevious\":{\"coordinates\":[[59.94525,18.13365],[59.94562,18.17644],[59.95361,18.18034],[59.96939,18.19872],[59.97882,18.21584]]}},\"stopArea\":{\"ref\":\"9021001010667000\",\"name\":\"Eriksbo\",\"shortName\":\"Eriksbo\"},\"stopPoint\":{\"ref\":\"9022001010667001\",\"name\":\"Eriksbo\",\"shortName\":\"Eriksbo\",\"designation\":\"\",\"localNumber\":1,\"stopPointKeys\":[{\"parameterData\":\"Värdshuset\",\"typeCode\":\"ADDITIONAL_INFO\",\"deviceName\":\"Alias\"}]},\"replacedJourneyPatternPoint\":{\"ref\":\"9025001000010688\",\"isTimingPoint\":true,\"location\":{\"latitude\":59.94525,\"longitude\":18.13365},\"distanceFromPrevious\":555,\"detection\":{\"enteringDistance\":20,\"exitingDistance\":10}},\"replacedStopArea\":{\"ref\":\"9021001010995000\",\"name\":\"Sickla väg\",\"shortName\":\"Sickla väg\"},\"replacedStopPoint\":{\"ref\":\"9022001010995004\",\"name\":\"Sickla väg\",\"shortName\":\"Sickla väg\",\"designation\":\"\",\"localNumber\":4},\"detourEnroute\":{\"startsAfterCallSequenceNumber\":1,\"instructions\":\"Driver instructions.\",\"path\":{\"coordinates\":[[59.94525,18.13365],[59.94352,18.14644],[59.95261,18.14034],[59.96439,18.16872],[59.97882,18.21584]]}},\"arrival\":{\"latestDateTime\":\"2017-04-27T05:53:00+02:00\",\"arrivalType\":\"STOP_IF_ALIGHTING\"},\"departure\":{\"earliestDateTime\":\"2017-04-27T05:53:00+02:00\",\"departureType\":\"STOP_IF_BOARDING\"},\"destinationDisplay\":{\"productName\":\"\",\"symbolName\":\"\",\"lineDesignation\":\"22B\",\"primaryDestination\":{\"name\":\"Thorildsplan\",\"shortName\":\"\"},\"secondaryDestination\":{\"name\":\"\",\"shortName\":\"\",\"secondaryDestinationType\":\"UNKNOWN\"},\"displayKeys\":[{\"parameterData\":\"Destination=659\",\"typeCode\":\"O_DESTIN\",\"deviceName\":\"SIGN\"}]},\"fetcherConnections\":[{\"connectionRef\":\"9014001002310000\",\"transportModeCode\":\"BUS\",\"lineAuthorityCode\":\"SL\",\"lineDesignation\":\"23\",\"directionName\":\"Mot Hötorget\",\"minChangeDurationSeconds\":120,\"maxWaitingUntilTime\":\"2017-04-27T06:05:00+02:00\"},{\"connectionRef\":\"9014001002410000\",\"transportModeCode\":\"BUS\",\"lineAuthorityCode\":\"SL\",\"lineDesignation\":\"24\",\"directionName\":\"Mot Rådhuset\",\"minChangeDurationSeconds\":60,\"maxWaitingUntilTime\":\"2017-04-27T06:03:00+02:00\"}]}]}";
}

