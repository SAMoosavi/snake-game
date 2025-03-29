use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Write},
};

use super::Board;

const JSON_FILE_PATH: &str = "./src/boards.json";

#[derive(Debug)]
pub struct Boards {
    boards: HashMap<String, Board>,
}

impl Boards {
    pub fn new() -> Self {
        let file = File::open(JSON_FILE_PATH).unwrap();

        let reader = BufReader::new(file);
        let boards: HashMap<String, Board> = serde_json::from_reader(reader).unwrap();

        Self { boards }
    }

    pub fn add(&mut self, name: String, board: Board) -> Result<(), String> {
        if self.boards.contains_key(&name) {
            return Err(format!("Board '{}' already exists", name));
        }

        self.boards.insert(name, board);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Board> {
        self.boards.get(name)
    }

    pub fn get_names(&self) -> Vec<String> {
        self.boards.keys().cloned().collect()
    }
}

impl Drop for Boards {
    fn drop(&mut self) {
        let json = serde_json::to_string(&self.boards).unwrap();
        let mut file = File::create(JSON_FILE_PATH).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
