use log::info;

#[macro_export]
macro_rules! handled{
    ($($arg:tt)+) => (info!(target: "handled", $($arg)+))
}

#[macro_export]
macro_rules! received{
    ($message: literal) => {
        info!("received: {$message}");
    }
}


