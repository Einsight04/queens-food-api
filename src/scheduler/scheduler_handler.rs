// use name_of_yourlib::components::header

use crate::services::food_info;
use clokwerk::Interval::*;
use clokwerk::{Scheduler, TimeUnits};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

pub fn handler(con: &mut r2d2::PooledConnection<redis::Client>) {
    let mut scheduler = Scheduler::new();

    // // every day at 12:00 am
    // scheduler.every(1.day()).at("00:00").run(|| {
    //     food_info::handler(&mut con);
    // });

    // borrowed data escapes outside of closure fix this
    // move into closure
    scheduler.every(1.day()).at("00:00").run(|| {
        food_info::handler(con);
    });
    thread::spawn(move || loop {
        scheduler.run_pending();
        let _ = sleep(Duration::from_millis(1 * 1000));
    });
}
