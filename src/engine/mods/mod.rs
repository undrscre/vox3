pub mod blocks;
pub mod texture;

const PACK_VERSION: u32 = 0;
use std::{fs, path::PathBuf};

use blocks::BlockRegistry;
// use texture::TextureRegistry;

use crate::{engine::mods::{blocks::BlockTextures, texture::TextureRegistry}, render::{device::GPUDevice, texture::TextureArray}};

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum TextureConfig {
    Simple { all: String },
    Detailed { top: String, bottom: String, side: String }
}

#[derive(serde::Deserialize)]
struct BlockJson {
    name: String,
    opaque: bool,
    textures: TextureConfig,
}

#[derive(serde::Deserialize, Debug)]
pub struct ModManifest {
    pub id: String,
    pub name: String,
    pub pack_version: u32,
    pub mod_version: String,
    pub priority: u32
}

pub struct LoadedMod {
    pub path: PathBuf,
    pub manifest: ModManifest,
}

pub fn discover_mods(root: &str) -> Vec<LoadedMod> {
    fs::read_dir(root)
        .expect("mods folder missing")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|entry| {
            let manifest_path = entry.path().join("mod.json");
            let file = fs::read_to_string(manifest_path).ok()?;
            let manifest: ModManifest = serde_json::from_str(&file).expect("mod manifest malformed");

            if manifest.pack_version > PACK_VERSION {
                log::warn!("skipping mod {} - version mismatch (expected < {})", manifest.id, PACK_VERSION);
                return None;
            }

            Some(LoadedMod {
                path: entry.path(),
                manifest
            })
        })
        .collect()
}

pub fn load_every_mod(root: &str) -> (TextureRegistry, BlockRegistry) {
    let mut mods = discover_mods(root);
    if mods.is_empty() {
        log::error!("!! NO MODS FOUND !! panicking!");
        panic!()
    }

    let mut texture_registry = TextureRegistry::new();

    mods.sort_by_key(|m| m.manifest.priority);
    for m in &mods {
        let asset_dir = m.path.join("assets");
        texture_registry.collect_from_folder(asset_dir);
    }

    // block definitions!!
    let mut block_registry = BlockRegistry::new();
    for m in &mods {
        let data_path = m.path.join("data/blocks.json");
        if data_path.exists() {
            let json: Vec<BlockJson> = serde_json::from_str(&std::fs::read_to_string(data_path).unwrap()).unwrap();
            for b in json {
                let tex = match b.textures {
                    TextureConfig::Simple { all } => {
                        let id = texture_registry.get(&all);
                        BlockTextures { top: id, bottom: id, sides: id }
                    }
                    TextureConfig::Detailed { top, bottom, side } => {
                        println!("loading block {} with textures {} {} {}", b.name, top, bottom, side);
                        BlockTextures {
                            top: texture_registry.get(&top),
                            bottom: texture_registry.get(&bottom),
                            sides: texture_registry.get(&side),
                        }
                    }
                };
                block_registry.register(&b.name, b.opaque, tex);
            }
        }
    }

    (texture_registry, block_registry)
}