// use name_of_yourlib::components::header

use clokwerk::Interval::*;
use clokwerk::{Scheduler, TimeUnits};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use redis::Commands;

pub fn start(con: &mut r2d2::PooledConnection<redis::Client>) {
    // run function everyday at 12:00
    let mut scheduler = Scheduler::new();

    // scheduler.every(1.day()).at("12:00").run(move || {
    //     food_info::handler(con);
    // });

    loop {
        scheduler.run_pending();
        sleep(Duration::from_secs(1));
    }
}
