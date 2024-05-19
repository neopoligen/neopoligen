use itertools::Itertools;
use notify_debouncer_full::{new_debouncer, notify::*, DebounceEventResult, Debouncer, FileIdMap};
use std::path::PathBuf;
use std::time::Duration;
use tokio::runtime::Handle;
use tokio::sync::mpsc;

pub struct FileWatcher {
    pub debounced_watcher: Debouncer<RecommendedWatcher, FileIdMap>,
}

impl FileWatcher {
    pub async fn new(path: &PathBuf, tx: mpsc::Sender<Vec<PathBuf>>) -> FileWatcher {
        let rt = Handle::current();
        let mut debouncer = new_debouncer(
            Duration::from_millis(300),
            None,
            move |result: DebounceEventResult| match result {
                Ok(events) => {
                    dbg!(".");
                    dbg!(&events);
                    let paths: Vec<_> = events
                        .iter()
                        .filter_map(|e| match e.event.kind {
                            EventKind::Create(..) => Some(e.paths.clone()),
                            EventKind::Modify(..) => Some(e.paths.clone()),
                            EventKind::Remove(..) => Some(e.paths.clone()),
                            _ => None,
                        })
                        .flatten()
                        .unique()
                        //.filter_map(|p| if p.is_file() { Some(p) } else { None })
                        .collect();
                    if paths.len() > 0 {
                        let tx = tx.clone();
                        rt.spawn(async move {
                            if let Err(e) = tx.send(paths).await {
                                println!("Error sending event result: {:?}", e);
                            }
                        });
                        ()
                    }
                }
                Err(e) => println!("{:?}", e),
            },
        )
        .unwrap();
        debouncer
            .watcher()
            .watch(path, RecursiveMode::Recursive)
            .unwrap();
        debouncer.cache().add_root(path, RecursiveMode::Recursive);
        FileWatcher {
            debounced_watcher: debouncer,
        }
    }
}
