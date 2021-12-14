use std::fs::{self, File};
use std::io::{Read, Write};

use crate::config::APP_NAME;

/*******************************************************************************
**** SCORING
*******************************************************************************/
pub struct Scoring {
    pub score: usize,

    pub maxcombo: usize,
    pub is_new_maxcombo: bool,
    maxcombo_accumulator: usize,

    pub highscore: usize,
    pub is_new_highscore: bool,
    is_new_highscore_already_reached: bool,
}

impl Scoring {
    const HIGHSCORE_DIRNAME: &'static str = APP_NAME;
    const HIGHSCORE_FILENAME: &'static str = "cc_hs";

    pub fn new(highscore: usize) -> Self {
        Scoring {
            score: 0,
            maxcombo: 0,
            is_new_maxcombo: false,
            maxcombo_accumulator: 0,
            highscore,
            is_new_highscore: false,
            is_new_highscore_already_reached: false,
        }
    }

    pub fn load_highscore() -> usize {
        let file_path = dirs::data_dir()
            .unwrap()
            .join(Self::HIGHSCORE_DIRNAME)
            .join(Self::HIGHSCORE_FILENAME);
        if let Ok(mut file) = File::open(file_path) {
            let mut str_buf = String::new();
            if file.read_to_string(&mut str_buf).is_ok() {
                return str_buf.parse::<usize>().unwrap();
            }
        }
        0
    }

    pub fn update_from_matches(
        &mut self,
        (num_of_matching_blocks, num_of_sequential_matchings): (&Vec<usize>, usize),
    ) {
        // SCORE
        let mut combo = 0;
        for num_matches in num_of_matching_blocks {
            let bonus_length = num_matches - 3;
            let mut bonus_points = 0;
            if bonus_length > 0 {
                for bonus_point in 2..bonus_length + 2 {
                    bonus_points += bonus_point;
                }
            }
            let points_per_match = 3 + bonus_points;
            combo += points_per_match;
        }
        combo = combo * num_of_matching_blocks.len() * num_of_sequential_matchings;
        self.score += combo;
        // MAX COMBO
        if num_of_sequential_matchings == 1 {
            self.maxcombo_accumulator = 0;
        }
        self.maxcombo_accumulator += combo;
        self.is_new_maxcombo = false;
        if combo > self.maxcombo {
            self.maxcombo = self.maxcombo_accumulator;
            self.is_new_maxcombo = true;
        }
        // HIGHSCORE
        self.is_new_highscore = false;
        if self.score > self.highscore && !self.is_new_highscore_already_reached {
            self.is_new_highscore = true;
            self.is_new_highscore_already_reached = true;
        }
    }

    pub fn save_highscore(&mut self) {
        if self.score <= self.highscore {
            return;
        }

        let mut file = None;

        let file_path = dirs::data_dir()
            .unwrap()
            .join(Self::HIGHSCORE_DIRNAME)
            .join(Self::HIGHSCORE_FILENAME);
        if file_path.exists() {
            if let Ok(truncated_file) = File::create(file_path) {
                file = Some(truncated_file);
            }
        } else {
            let mut data_dir_path = dirs::data_dir().unwrap();
            data_dir_path.push(Self::HIGHSCORE_DIRNAME);
            if !data_dir_path.exists() {
                if fs::create_dir_all(&data_dir_path).is_ok() {
                    let file_path = data_dir_path.join(Self::HIGHSCORE_FILENAME);
                    if let Ok(created_file) = File::create(file_path) {
                        file = Some(created_file);
                    } else {
                        eprintln!("Highscore file could not be created!");
                    }
                } else {
                    eprintln!("Highscore directory could not be created!");
                }
            }
        }

        if let Some(mut file) = file {
            let write_result = file.write_all(self.score.to_string().as_bytes());
            if write_result.is_err() {
                eprintln!(
                    "Could not write highscore to file! {:?}",
                    write_result.unwrap_err()
                );
            }
        }
    }
}
