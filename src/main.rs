mod api;

use api::*;
use std::io::Read;
use rand::*;
use rand::rngs::ThreadRng;
use std::{thread, time};
use std::thread::Thread;

fn main() {
    init();

    let j = thread::spawn(|| epilepsy());
    j.join();

    shutdown();
}

fn epilepsy() {
    let mut rng = rand::thread_rng();
    for _ in 0..500 {
        println!("generating");
        thread::sleep(time::Duration::from_millis(10));
        let _ = set_lightning(rng.gen_range(0..100), rng.gen_range(0..100), rng.gen_range(0..100));
    }
}