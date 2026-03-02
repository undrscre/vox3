use std::path::Path;
use std::fs;

use crate::engine::data::{CHUNK_SIZE, BlockTypes};

const CHUNK_MAGIC_NUMBER: u32 = 1835361128;
struct ChunkFile {
    version: u32,

    x_pos: i16,
    y_pos: i16,
    z_pos: i16,
    hash: u32,

    data: [BlockTypes; CHUNK_SIZE]
}

struct ChunkLoader {
    
}

// impl ChunkLoader {
//     fn new(save_dir: &Path) -> Result<Self, std::io::Error> {
//         let dir = fs::read_dir(save_dir);
//         log::info!("reading chunks {:?} from directory", save_dir);
//         if let Ok(result) = dir {
            
//         } else {
//             log::error!("no directory found! creating");
//             let new_dir = fs::DirBuilder::create(&self, path)
//         }
        
//         todo!()
//     }
// }