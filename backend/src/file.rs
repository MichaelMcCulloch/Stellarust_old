use std::thread::{self, JoinHandle};

pub fn do_a_thing() {
    println!("I AM A FILE SERVICE")
}

pub fn start_file_service() -> JoinHandle<()> {
    thread::spawn(|| {
        do_a_thing();
    })
}
