

pub struct Config {
    port: u16,    
}

impl Config {
    pub fn new (port: u16) -> Self {
        Config {
            port
        }
    }
    pub fn port(&self) -> u16 {
        return self.port;
    }
}

