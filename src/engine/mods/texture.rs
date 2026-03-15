use std::{fs, path::PathBuf};
use rustc_hash::FxHashMap;

pub struct TextureRegistry {
    pub name_to_id: FxHashMap<String, u16>,
    pub paths: Vec<PathBuf>,
    pub count: u16,
}

impl TextureRegistry {
    pub fn new() -> Self {
        let mut s = Self { name_to_id: FxHashMap::default(), count: 0, paths: vec![] };
        s.name_to_id.insert("internal:missing".to_string(), 0);
        s.count += 1;
        s
    }

    pub fn collect_from_folder(&mut self, folder: PathBuf) {
        if let Ok(entries) = fs::read_dir(folder) {
            for entry in entries.filter_map(|e| e.ok()) {
                if entry.path().extension().map_or(false, |ext| ext == "png") {
                    // AHHHHHHHHHHHHHHHHHHH
                    let name = entry.path().file_stem().unwrap().to_str().unwrap().to_string();

                    if let Some(&idx) = self.name_to_id.get(&name) {
                        self.paths[idx as usize] = entry.path();
                    } else {
                        let new_idx = self.paths.len() as u16;
                        self.name_to_id.insert(name, new_idx);
                        self.paths.push(entry.path());
                        self.count += 1;
                    }
                }
            }
        }
    }

    pub fn get(&self, name: &str) -> u16 {
        *self.name_to_id.get(name).unwrap_or(&0) // fallback to checkerboard
    }
}