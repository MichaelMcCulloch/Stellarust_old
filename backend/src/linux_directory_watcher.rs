use notify::{raw_watcher, INotifyWatcher, RawEvent, RecursiveMode, Watcher};

use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
pub struct DirectoryWatcher {
    watcher: INotifyWatcher,
    pub rx: Receiver<RawEvent>,
}

impl DirectoryWatcher {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let watcher = raw_watcher(tx.clone()).unwrap();
        DirectoryWatcher { watcher, rx }
    }

    pub fn create(watch_dir: PathBuf) -> Self {
        let mut dir_watcher = DirectoryWatcher::new();

        dir_watcher
            .watcher
            .watch(watch_dir, RecursiveMode::NonRecursive)
            .unwrap();

        dir_watcher
    }
}
