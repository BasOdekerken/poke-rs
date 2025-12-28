use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct JsonMap {
    pub version: u32,
    pub tile_size: u32,
    pub width: u32,
    pub height: u32,
    pub legend: HashMap<char, JsonTileDef>,
    pub rows: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JsonTileDef {
    pub blocked: bool,
    pub encounter: bool,
    pub color: [f32; 3],
}

impl JsonMap {
    pub fn load(path: &str) -> Self {
        let text = std::fs::read_to_string(path).expect("Failed to read map file");
        serde_json::from_str(&text).expect("Failed to parse map JSON")
    }

    pub fn validate(&self) {
        assert!(
            self.rows.len() == self.height as usize,
            "Row counts does not match height"
        );

        for (y, row) in self.rows.iter().enumerate() {
            assert!(
                row.chars().count() == self.width as usize,
                "Row {} has incorrect length",
                y
            );

            for ch in row.chars() {
                assert!(
                    self.legend.contains_key(&ch),
                    "Legend missing tile '{}'",
                    ch
                );
            }
        }
    }
}
