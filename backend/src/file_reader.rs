use actix_web::middleware;
use core::panic;
use notify::Op;
use notify::RawEvent;
use std::{
    env,
    fs::File,
    io::Read,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};
pub struct FileReader {
    tx: Sender<String>,
    pub rx: Receiver<String>,
}

impl FileReader {
    fn new() -> Self {
        let (tx, rx) = channel();
        FileReader { tx, rx }
    }

    pub fn create(directory_watcher_rx: Receiver<RawEvent>) -> Self {
        let me = FileReader::new();
        me.start_file_reader_thread(directory_watcher_rx);
        me
    }

    fn start_file_reader_thread(&self, directory_watcher_rx: Receiver<RawEvent>) {
        let sender = self.tx.clone();
        actix_web::rt::spawn(async move {
            loop {
                let path = match directory_watcher_rx.recv() {
                    Ok(RawEvent {
                        path: Some(path),
                        op: Ok(Op::CLOSE_WRITE),
                        cookie: _,
                    }) => path,
                    Ok(_) => continue,
                    Err(e) => {
                        log::error!("{}", e);
                        continue;
                    }
                };

                let mut file = match File::open(path) {
                    Ok(file) => file,
                    Err(e) => {
                        log::error!("{}", e);
                        continue;
                    }
                };

                let mut content = String::new();
                match file.read_to_string(&mut content) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        log::error!("{}", e);
                        continue;
                    }
                };

                log::info!("Sending: {}", content);

                match sender.send(content) {
                    Ok(()) => {}
                    Err(e) => {
                        log::error!("{}", e);
                        continue;
                    }
                }
            }
        })
    }
}
