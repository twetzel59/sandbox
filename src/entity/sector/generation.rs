//! Provides the game's multithreaded world generator.

use super::{
    data::{SectorCoords, SectorData, SECTOR_MAX},
    SectorIndex,
};
use crate::block::Block;
use std::{
    mem,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
};

// The number of worker threads that should run
// in the background, in addition to the main thread.
const N_WORKERS: usize = 1;

/// Manages generation workers.
///
/// This ``struct`` stores the handles and channels
/// for the worker threads and provides methods to
/// request sector generation or final cleanup.
pub struct GenController {
    rx: Option<Receiver<Message>>,
    handles: Vec<JoinHandle<()>>,
}

impl GenController {
    /// Create a new world generation controller
    /// and start worker threads.
    ///
    /// This method will create ``n_workers``
    /// background threads.
    pub fn launch() -> GenController {
        let (tx, rx) = mpsc::channel();

        GenController {
            rx: Some(rx),
            handles: Self::spawn_threads(tx, N_WORKERS),
        }
    }

    fn spawn_threads(tx: Sender<Message>, n: usize) -> Vec<JoinHandle<()>> {
        let mut handles = Vec::new();

        for _ in 0..n {
            let tx = tx.clone();

            handles.push(thread::spawn(move || {
                worker_thread(tx);
            }));
        }

        handles
    }
}

impl Drop for GenController {
    fn drop(&mut self) {
        mem::drop(self.rx.take());
        
        for i in self.handles.drain(..) {
            i.join();
        }
    }
}

struct Message {
    world_pos: SectorIndex,
    data: SectorData,
}

fn worker_thread(tx: Sender<Message>) {
    for x in -300..301 {
        for y in -1..0 {
            for z in -300..301 {
                let world_pos = SectorIndex(x, y, z);
                let data = superflat_sector(world_pos);

                let message = Message { world_pos, data };

                match tx.send(message) {
                    Err(_) => {
                        println!("quitting!");
                        return;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn superflat_sector(world_pos: SectorIndex) -> SectorData {
    let mut data = SectorData::new();

    if world_pos.1 != -1 {
        return data;
    }

    for (SectorCoords(_, y, _), blk) in data.iter_mut() {
        *blk = if y == SECTOR_MAX {
            Block::Grass
        } else {
            Block::Soil
        };
    }

    data
}
