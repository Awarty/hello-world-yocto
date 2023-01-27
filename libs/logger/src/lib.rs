#[macro_use]
mod macros;

use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::{json::JsonEncoder, pattern::PatternEncoder}
};
use log::{info, LevelFilter};

pub fn init_logger() {

    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{h({d} {l} {M} {t} - {m} \t {n})}")))
        //.encoder(Box::new(JsonEncoder::new()))
        .build();
    let log_config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .unwrap();

    //log4rs::init_file("configs/log4rs.yaml", Default::default()).unwrap();
    log4rs::init_config(log_config).unwrap();

    info!("Logger initilized");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
