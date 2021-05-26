use std::{
    fs::File,
    io::Read,
    path::PathBuf,
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

    pub fn create(directory_watcher_rx: Receiver<PathBuf>) -> Self {
        let me = FileReader::new();
        me.start_file_reader_thread(directory_watcher_rx);
        me
    }

    fn start_file_reader_thread(&self, directory_watcher_rx: Receiver<PathBuf>) {
        let sender = self.tx.clone();

        thread::spawn(move || loop {
            match loop_iter(&directory_watcher_rx, &sender) {
                Err(e) => log::error!("{}", e),
                _ => continue,
            }
        });
    }
}

fn loop_iter(
    directory_watcher_rx: &Receiver<PathBuf>,
    sender: &Sender<String>,
) -> Result<(), anyhow::Error> {
    let path = directory_watcher_rx.recv()?;
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    sender.send(content)?;
    Ok(())
}
