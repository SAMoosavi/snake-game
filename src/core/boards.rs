use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use super::Board;

const JSON_FILE_PATH: &str = "./src/boards.json";

#[derive(Debug)]
pub struct Boards {
    boards: HashMap<String, Board>,
    file_path: PathBuf,
}

impl Boards {
    pub fn load() -> Boards {
        let json_file_path = Path::new(JSON_FILE_PATH);

        let file = File::open(json_file_path).unwrap();

        let reader = BufReader::new(file);
        let boards: HashMap<String, Board> = serde_json::from_reader(reader).unwrap();

        Self {
            boards,
            file_path: json_file_path.to_path_buf(),
        }
    }

    pub fn add(&mut self, name: String, board: Board) -> Result<(), String> {
        if self.boards.contains_key(&name) {
            Err(format!("the name of {} exist", name))
        } else {
            self.boards.insert(name, board);
            Ok(())
        }
    }
    pub fn get(&self, name: &str) -> Option<&Board> {
        self.boards.get(name)
    }

    pub fn get_names(&self) -> Vec<String> {
        self.boards.keys().cloned().collect()
    }

    pub fn save(&self) {
        let j = serde_json::to_string(&self.boards).unwrap();
        let mut file = File::create(self.file_path.to_path_buf()).unwrap();
        file.write_all(j.as_bytes()).unwrap();
    }
}
