use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Write},
};

const JSON_FILE_PATH: &str = "./src/scoreboard.json";

type ScoreboardType = HashMap<String, Vec<u16>>;

#[derive(Debug)]
pub struct Scoreboard {
    scoreboard: ScoreboardType,
}

impl Scoreboard {
    pub fn new() -> Self {
        let file = File::open(JSON_FILE_PATH).unwrap();

        let reader = BufReader::new(file);
        let scoreboard: ScoreboardType = serde_json::from_reader(reader).unwrap();
        Self { scoreboard }
    }

    pub fn add(&mut self, board_name: String, score: u16) {
        self.scoreboard.entry(board_name).or_default().push(score);
    }

    pub fn get(&self, board_name: &str) -> Option<Vec<u16>> {
        self.scoreboard.get(board_name).map(|s| {
            let mut s = s.clone();
            s.sort_unstable();
            s.reverse();
            s
        })
    }
}

impl Drop for Scoreboard {
    fn drop(&mut self) {
        let json = serde_json::to_string(&self.scoreboard).unwrap();
        let mut file = File::create(JSON_FILE_PATH).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
