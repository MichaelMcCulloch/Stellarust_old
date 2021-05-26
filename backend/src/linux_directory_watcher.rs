use notify::{raw_watcher, INotifyWatcher, Op, RawEvent, RecursiveMode, Watcher};

use std::sync::mpsc::{channel, Receiver, Sender};
use std::{path::PathBuf, thread};
pub struct DirectoryWatcher {
    watcher: INotifyWatcher,
    pub pathbuf_receiver: Receiver<PathBuf>,
}

impl DirectoryWatcher {
    pub fn create(watch_dir: PathBuf) -> Self {
        let (raw_event_sender, raw_event_receiver) = channel();
        let (pathbuf_sender, pathbuf_receiver) = channel();
        let watcher = raw_watcher(raw_event_sender.clone()).unwrap();

        let mut dir_watcher = DirectoryWatcher {
            watcher,
            pathbuf_receiver,
        };

        dir_watcher
            .watcher
            .watch(watch_dir, RecursiveMode::NonRecursive)
            .unwrap();

        thread::spawn(move || loop {
            match loop_iter(&raw_event_receiver, &pathbuf_sender) {
                Err(e) => log::error!("{}", e),
                _ => continue,
            }
        });

        dir_watcher
    }
}

fn loop_iter(
    raw_event_receiver: &Receiver<RawEvent>,
    pathbuf_sender: &Sender<PathBuf>,
) -> Result<(), anyhow::Error> {
    let event = raw_event_receiver.recv()?;

    (match event {
        RawEvent {
            op: Ok(Op::CLOSE_WRITE),
            path: Some(path),
            cookie: _cookie,
        } => pathbuf_sender.send(path),
        _ => Ok(()),
    })?;

    Ok(())
}
