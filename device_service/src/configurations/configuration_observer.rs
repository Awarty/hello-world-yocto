pub trait ConfigurationObserver {
    fn path(&self) -> String;
    fn notify(&self, data: &str);
}
