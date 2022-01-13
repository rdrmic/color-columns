#![allow(clippy::missing_const_for_fn)]

use std::fs::{self, File};
use std::io::{Read, Write};

use ggez::GameError;

use crate::app::log_error;
use crate::constants::APP_NAME;

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
        Self {
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
        fn notify_about_error(error_msg: String) {
            eprintln!("{}", error_msg);
            log_error("load_highscore", &GameError::CustomError(error_msg));
        }

        let mut highscore = 0;
        if let Some(user_data_dir_path) = dirs::data_dir() {
            let file_path = user_data_dir_path
                .join(Self::HIGHSCORE_DIRNAME)
                .join(Self::HIGHSCORE_FILENAME);
            if let Ok(mut file) = File::open(file_path) {
                let mut str_buf = String::new();
                match file.read_to_string(&mut str_buf) {
                    Ok(_) => match str_buf.parse::<usize>() {
                        Ok(parsed_highscore) => highscore = parsed_highscore,
                        Err(error) => notify_about_error(format!(
                            "Highscore could not be parsed from the file: {:?}",
                            error
                        )),
                    },
                    Err(error) => {
                        notify_about_error(format!(
                            "Highscore file could not be read: {:?}",
                            error
                        ));
                    }
                }
            } else {
                eprintln!("Highscore file could not be open. Maybe it doesn't exist yet?");
            }
        } else {
            notify_about_error(
                "User's data dir is not found, the highscore could not be loaded".to_string(),
            );
        }
        highscore
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
        if self.maxcombo_accumulator > self.maxcombo {
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
        fn notify_about_error(error_msg: String) {
            eprintln!("{}", error_msg);
            log_error("save_highscore", &GameError::CustomError(error_msg));
        }

        if self.score <= self.highscore {
            return;
        }

        let mut file = None;

        if let Some(user_data_dir_path) = dirs::data_dir() {
            let file_path = user_data_dir_path
                .join(Self::HIGHSCORE_DIRNAME)
                .join(Self::HIGHSCORE_FILENAME);
            if file_path.exists() {
                if let Ok(truncated_file) = File::create(file_path) {
                    file = Some(truncated_file);
                }
            } else {
                let dir_path = user_data_dir_path.join(Self::HIGHSCORE_DIRNAME);
                if !dir_path.exists() {
                    match fs::create_dir_all(&dir_path) {
                        Ok(_) => {
                            let file_path = dir_path.join(Self::HIGHSCORE_FILENAME);
                            match File::create(file_path) {
                                Ok(created_file) => file = Some(created_file),
                                Err(error) => notify_about_error(format!(
                                    "Highscore file could not be created: {:?}",
                                    error
                                )),
                            }
                        }
                        Err(error) => notify_about_error(format!(
                            "Highscore directory could not be created: {:?}",
                            error
                        )),
                    }
                }
            }
        } else {
            notify_about_error(
                "User's data dir is not found, the highscore could not be saved".to_string(),
            );
        }

        if let Some(mut file) = file {
            let write_result = file.write_all(self.score.to_string().as_bytes());
            if write_result.is_err() {
                eprintln!(
                    "Could not write the highscore to the file: {:?}",
                    write_result.unwrap_err()
                );
            }
        } else {
            notify_about_error("Failed to create a highscore file".to_string());
        }
    }
}
