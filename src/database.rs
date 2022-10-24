extern crate redis;

use redis::Commands;

pub const CLIENT: redis::Client = redis::Client::open("redis://127.0.0.1:6379")
    .expect("Failed to connect to redis");