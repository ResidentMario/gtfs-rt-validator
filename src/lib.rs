pub mod err;
pub mod report;

pub mod transit_realtime {
    include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));
}
