use crate::log::Log;

pub trait Analyze {
    fn group_by_level(logs: Vec<Log>) -> Vec<Log>;
    fn group_by_time(logs: Vec<Log>) -> Vec<Log>;
}