use std::{
    fs::File,
    io::Read,
    path::PathBuf,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};
pub struct FileReader {
    pub file_content_receiver: Receiver<String>,
}

impl FileReader {
    pub fn create(pathbuf_receiver: Receiver<PathBuf>) -> Self {
        let (file_content_sender, file_content_receiver) = channel();

        let file_reader = FileReader {
            file_content_receiver,
        };
        file_reader.start_file_reader(pathbuf_receiver, file_content_sender);
        file_reader
    }

    fn start_file_reader(
        &self,
        pathbuf_receiver: Receiver<PathBuf>,
        file_content_sender: Sender<String>,
    ) {
        thread::spawn(move || loop {
            match forward_path_to_contents(&pathbuf_receiver, &file_content_sender) {
                Err(e) => log::error!("{}", e),
                _ => continue,
            }
        });
    }
}

fn forward_path_to_contents(
    pathbuf_receiver: &Receiver<PathBuf>,
    file_content_sender: &Sender<String>,
) -> Result<(), anyhow::Error> {
    let path = pathbuf_receiver.recv()?;
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    file_content_sender.send(content)?;
    Ok(())
}
