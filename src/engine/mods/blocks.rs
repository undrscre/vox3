use std::collections::HashMap;

// block registry
#[derive(Clone)]
pub struct BlockTextures {
    pub top: u16,
    pub bottom: u16,
    pub sides: u16
}


impl BlockTextures {
    pub fn empty() -> Self {
        Self { top: 0, bottom: 0, sides: 0 }
    }
}

#[derive(Clone)]
pub struct BlockDefinition {
    pub name: String,
    pub opaque: bool,
    pub textures: BlockTextures
}

#[derive(Clone)]
pub struct BlockRegistry {
    pub definitions: Vec<BlockDefinition>,
    pub name_to_id: HashMap<String, u16>
}

impl BlockRegistry {
    pub fn new() -> Self {
        let mut s = BlockRegistry {
            definitions: Vec::with_capacity(256),
            name_to_id: HashMap::new()
        };
        
        s.register("air", false, BlockTextures::empty());
        s
    }

    pub fn register(&mut self, name: &str, opaque: bool, textures: BlockTextures) -> u16 {
        let id = self.definitions.len() as u16;
        self.definitions.push(BlockDefinition { name: name.to_string(), opaque, textures });
        self.name_to_id.insert(name.to_string(), id);
        id
    }
}