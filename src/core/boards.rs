use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

use super::Board;

#[derive(Debug)]
pub struct Boards {
    boards: HashMap<String, Board>,
}

impl Boards {
    pub fn load(file_path: &str) -> Boards {
        let json_file_path = Path::new(file_path);

        let file = File::open(json_file_path).unwrap();

        let reader = BufReader::new(file);
        let boards: HashMap<String, Board> = serde_json::from_reader(reader).unwrap();

        Self { boards }
    }

    pub fn get(&self, name: &str) -> Option<&Board> {
        self.boards.get(name)
    }

    pub fn get_names(&self) -> Vec<String> {
        self.boards.keys().cloned().collect()
    }

    pub fn save(file_path: &str, board: HashMap<&str, Board>) {
        let j = serde_json::to_string(&board).unwrap();
        let json_file_path = Path::new(file_path);
        println!("{:?}", json_file_path);

        let mut file = File::create(json_file_path).unwrap();
        file.write_all(j.as_bytes()).unwrap();
    }
}
