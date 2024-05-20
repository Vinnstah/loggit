use crate::log::Log;

pub trait Logger {
    fn new_log(&self, gateway: dyn Gateway) -> Log;
}

pub trait Gateway {
    fn upload_log(&self) -> Result<(), ()>;
    fn retrieve_logs(&self) -> Result<Vec<Log>, ()>;
}
