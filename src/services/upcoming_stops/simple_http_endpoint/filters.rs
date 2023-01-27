use std::sync::{Arc, Mutex};

use super::handlers;
use super::super::data_handler;

use warp::Filter;
use std::collections::HashMap;

pub fn endpoints(data_handler: Arc<Mutex<data_handler::DataHandler>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    destinationtext(data_handler.clone())
        .or(calls(data_handler.clone()))
        .or(display_mode(data_handler.clone()))
}

fn destinationtext(data_handler: Arc<Mutex<data_handler::DataHandler>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("destinationtext")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(with_data_handler(data_handler))
        .and_then(handlers::destinationtext)
}

fn calls(data_handler: Arc<Mutex<data_handler::DataHandler>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("calls")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(with_data_handler(data_handler))
        .and_then(handlers::calls)
}

fn display_mode(data_handler: Arc<Mutex<data_handler::DataHandler>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("displayMode")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(with_data_handler(data_handler))
        .and_then(handlers::display_mode)
}

fn with_data_handler(param: Arc<Mutex<data_handler::DataHandler>>) ->
impl Filter<Extract = (Arc<Mutex<data_handler::DataHandler>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || param.clone())
} 


