use std::convert::Infallible;
use super::super::data_handler;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use log::{debug, error};
use warp::hyper::StatusCode;
use warp::Reply;

pub async fn destinationtext(hash: HashMap<String, String>, data_handler: Arc<Mutex<data_handler::DataHandler>>) -> Result<warp::reply::Response, Infallible> {
    let handler_lock = data_handler.lock();
    match handler_lock {
        Ok(handler) => {
            if let Some(destination) = handler.destination() {
                if hash.contains_key("hash") && destination.verify_hash(&hash["hash"]) {
                    debug!("No new data. Same hash as the old data.");
                    return Ok(warp::reply::json(&destination.get_hash()).into_response());
                }
                debug!("Responded with 'destination text' data.");
                return Ok(warp::reply::json(&destination).into_response());
            }
            else {
                Ok(warp::reply::with_status(
                    format!("The 'DestinationViewModel' is None."),
                    StatusCode::NOT_FOUND, 
                ).into_response())
            }

        }
        Err(e) => {
            error!("Failed to handle destinationtext {e:?}");
            return Ok(warp::reply::with_status(
                format!("Failed to handle destinationtext, {e:?}"),
                StatusCode::INTERNAL_SERVER_ERROR, 
            ).into_response());
        }
    }
}

pub async fn calls(hash: HashMap<String, String>, data_handler: Arc<Mutex<data_handler::DataHandler>>) -> Result<warp::reply::Response, Infallible> {
    let handler_lock = data_handler.lock();
    match handler_lock {
        Ok(handler) => {
            if let Some(calls) = handler.calls() {
                if hash.contains_key("hash") && calls.verify_hash(&hash["hash"]) {
                    debug!("No new data. Same hash as the old data.");
                    return Ok(warp::reply::json(&calls.get_hash()).into_response());
                }
                debug!("Responded with 'calls' data.");
                return Ok(warp::reply::json(&calls).into_response());
            }
            else {
                debug!("The 'CallsViewModel' is None");
                Ok(warp::reply::with_status(
                    format!("The 'CallsViewModel' is None."),
                    StatusCode::NOT_FOUND, 
                ).into_response())
            }

        }
        Err(e) => {
            error!("Failed to handle calls {e:?}");
            return Ok(warp::reply::with_status(
                format!("Failed to handle calls, {e:?}"),
                StatusCode::INTERNAL_SERVER_ERROR, 
            ).into_response());
        }
    }
}

pub async fn display_mode(hash: HashMap<String, String>, data_handler: Arc<Mutex<data_handler::DataHandler>>) -> Result<warp::reply::Response, Infallible> {
    let handler_lock = data_handler.lock();
    match handler_lock {
        Ok(handler) => {
            if let Some(display_mode) = handler.display_mode() {
                if hash.contains_key("hash") && display_mode.verify_hash(&hash["hash"]) {
                    debug!("No new data. Same hash as the old data.");
                    return Ok(warp::reply::json(&display_mode.get_hash()).into_response());
                }
                debug!("Responded with 'display mode' data.");
                return Ok(warp::reply::json(&display_mode).into_response());
            }
            else {
                debug!("The 'DisplayVewMode' is None");
                Ok(warp::reply::with_status(
                    format!("The 'DisplayVewMode' is None."),
                    StatusCode::NOT_FOUND, 
                ).into_response())
            }

        }
        Err(e) => {
            error!("Failed to handle display mode {e:?}");
            return Ok(warp::reply::with_status(
                format!("Failed to handle display mode, {e:?}"),
                StatusCode::INTERNAL_SERVER_ERROR, 
            ).into_response());
        }
    }

}
