// use name_of_yourlib::components::header

use crate::scheduler::tasks::food_info_updater;

use std::thread;
use std::thread::sleep;
use std::time::Duration;
use clokwerk::{Scheduler, TimeUnits};
use clokwerk::Interval::*;

pub fn setup_scheduler() {
    let mut scheduler = Scheduler::new();

    // every day at 12:00 am
    scheduler.every(1.day()).at("00:00").run(|| {
        food_info_updater::handler();
    });

    thread::spawn(move || {
        loop {
            scheduler.run_pending();
            let _ = sleep(Duration::from_millis( 1 * 1000));
        }
    });
}