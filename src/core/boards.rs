use std::{
    fs::File,
    io::{BufReader, Write},
};

use super::Board;

const JSON_FILE_PATH: &str = "./src/boards.json";

#[derive(Debug)]
pub struct Boards {
    boards: Vec<Board>,
}

impl Boards {
    pub fn new() -> Self {
        let file = File::open(JSON_FILE_PATH).unwrap();

        let reader = BufReader::new(file);
        let boards: Vec<Board> = serde_json::from_reader(reader).unwrap();

        Self { boards }
    }

    pub fn add(&mut self, name: String, board: Board) -> Result<(), String> {
        if self.boards.iter().any(|board| board.get_name() == &name) {
            return Err(format!("Board '{}' already exists", name));
        }

        self.boards.push(board);
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&Board> {
        self.boards.get(index)
    }

    pub fn get_names(&self) -> Vec<String> {
        self.boards
            .iter()
            .map(|board| board.get_name().to_string())
            .collect()
    }
}

impl Drop for Boards {
    fn drop(&mut self) {
        let json = serde_json::to_string(&self.boards).unwrap();
        let mut file = File::create(JSON_FILE_PATH).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
