//! Provides the game's multithreaded world generator.

use super::{
    data::{SectorCoords, SectorData, SECTOR_MAX},
    meshgen::{self, PreGeometry},
    SectorIndex,
};
use crate::block::Block;
use png::OutputInfo;
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
    ///
    /// Since this manager delegates to workers
    /// that generate textured geometry, it needs
    /// access to texture metadata.
    pub fn launch(tex_info: &OutputInfo) -> GenController {
        let (tx, rx) = mpsc::channel();

        GenController {
            rx: Some(rx),
            handles: Self::spawn_threads(tx, tex_info, N_WORKERS),
        }
    }

    /// Return a reference to the ``Receiver`` over
    /// which new pre-generated ``Sector``s will be
    /// made available as ``Message`` instances.
    pub fn receiver(&self) -> &Receiver<Message> {
        self.rx.as_ref().unwrap()
    }

    fn spawn_threads(tx: Sender<Message>, tex_info: &OutputInfo, n: usize) -> Vec<JoinHandle<()>> {
        let mut handles = Vec::new();

        for _ in 0..n {
            let tx = tx.clone();
            let tex_info = copy_tex_info(tex_info);

            handles.push(thread::spawn(move || {
                worker_thread(tx, tex_info);
            }));
        }

        handles
    }
}

impl Drop for GenController {
    fn drop(&mut self) {
        mem::drop(self.rx.take());

        for i in self.handles.drain(..) {
            i.join().unwrap();
        }
    }
}

/// Stores the data created by the worker threads.
///
/// Includes the world position of the partially
/// constructed sector, its terrain data, and
/// — optionally — its pre-geometry.
pub struct Message {
    pub world_pos: SectorIndex,
    pub sector_data: SectorData,
    pub pre_geometry: Option<PreGeometry>,
}

fn worker_thread(tx: Sender<Message>, tex_info: OutputInfo) {
    for x in -10..11 {
        for y in -1..0 {
            for z in -10..11 {
                let world_pos = SectorIndex(x, y, z);
                let sector_data = superflat_sector(world_pos);

                let pre_geometry = meshgen::gen_terrain(&tex_info, &sector_data);

                let message = Message {
                    world_pos,
                    sector_data,
                    pre_geometry,
                };

                match tx.send(message) {
                    Ok(_) => {}
                    Err(_) => {
                        println!("quitting!");
                        return;
                    }
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

    for (SectorCoords(x, y, z), blk) in data.iter_mut() {
        *blk = if y < SECTOR_MAX - 1 {
            Block::Soil
        } else if y == SECTOR_MAX - 1 {
            if x % 4 == 0 && z % 4 == 0 {
                Block::TestBlock
            } else {
                Block::Grass
            }
        } else {
            Block::Air
        };
    }

    data
}

// The ``png`` crate does not include a ``Clone`` implementation
// for ``OutputInfo``, but it's fairly easy to reconstruct one.
fn copy_tex_info(tex_info: &OutputInfo) -> OutputInfo {
    OutputInfo {
        width: tex_info.width,
        height: tex_info.height,
        color_type: tex_info.color_type,
        bit_depth: tex_info.bit_depth,
        line_size: tex_info.line_size,
    }
}
