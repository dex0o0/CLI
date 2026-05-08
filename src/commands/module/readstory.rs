use super::smalmodule::sleep;
use crossterm::style::Stylize;
use std::{fs, path::PathBuf, process::Command};

pub fn read_file_story(path: PathBuf) {
    let _ = Command::new("clear").spawn();
    sleep(1000);
    let content = fs::read_to_string(&path).expect("Error to read file?!");
    let mut count: u8 = 0;
    let mut rang: char = ' ';
    for line in content.lines() {
        for char in line.chars() {
            if count == 3 {
                count = 0;
            };
            if char == '_' {
                sleep(500);
                eprint!(" ");
                continue;
            } else if char == ' ' {
                sleep(80);
                eprint!("{}", char);
                continue;
            } else if char == '\\' {
                if count != 3 {
                    count += 1;
                    continue;
                } else {
                    count = 0;
                    continue;
                }
            }
            if count == 1 {
                rang = char;
                continue;
            } else if count == 2 {
                match &rang {
                    'r' => eprint!("{}", char.red()),
                    'b' => eprint!("{}", char.blue()),
                    'y' => eprint!("{}", char.yellow()),
                    'g' => eprint!("{}", char.green()),
                    'B' => eprint!("{}", char.bold()),
                    _ => {}
                }
                continue;
            }

            eprint!("{}", char);
            sleep(80);
        }
        println!();
    }
}
