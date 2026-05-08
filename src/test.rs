use crossterm::style::Stylize;
use std::{fs, path::PathBuf, process::Command, thread, time::Duration};

fn main() {
    read_file_story_v2(PathBuf::from("./story"));
}

fn sleep(duration: u64) {
    thread::sleep(Duration::from_millis(duration));
}

fn get_command(not: String) -> String {
    not.replace("\"", "").replace("_", " ")
}

fn print(word: String) {
    let char = word.chars();
    char.for_each(|c| {
        eprint!("{}", c);
        sleep(50);
    });
    eprint!(" ");
}

fn sleep_cmd(word: String) -> Option<u32> {
    let start = word.find('(').unwrap();
    let end = word.find(')').unwrap();

    let num = &word[start + 1..end];
    num.parse().ok()
}

fn read_file_story_v2(path: PathBuf) {
    let _ = clearscreen::clear();
    let content = fs::read_to_string(&path).expect("i can't read file");
    content.lines().for_each(|line| {
        line.split_whitespace().for_each(|word| {
            sleep(100);
            if word == "." {
                eprint!("{}", word);
                return;
            }
            if word.starts_with("sleep(") && word.ends_with(")") {
                if let Some(dur) = sleep_cmd(word.to_string()) {
                    sleep(dur.into());
                }
                return;
            }
            if word.starts_with("\"") && word.ends_with("\".blue()") {
                let word_beatwin = get_command(word.to_string()).replace(".blue()", "");
                print(word_beatwin.blue().to_string());
                return;
            }
            if word.starts_with("\"") && word.ends_with("\".red()") {
                let word_beatwin = get_command(word.to_string()).replace(".red()", "");
                print(word_beatwin.red().to_string());
                return;
            }
            if word.starts_with("\"") && word.ends_with("\".green()") {
                let word_beatwin = get_command(word.to_string()).replace(".green()", "");
                print(word_beatwin.green().to_string());
                return;
            }
            if word.starts_with("\"") && word.ends_with("\".yellow()") {
                let word_beatwin = get_command(word.to_string()).replace(".yellow()", "");
                print(word_beatwin.yellow().to_string());
                return;
            }
            if word.starts_with("\"") && word.ends_with("\".bold()") {
                let word_beatwin = get_command(word.to_string()).replace(".bold()", "");
                print(word_beatwin.bold().to_string());
                return;
            }
            if word.contains("_") {
                let word_space = get_command(word.to_string());
                print(word_space);
                return;
            }
            print(word.to_string());
        });
        println!();
    });
}

#[allow(dead_code)]
fn read_file_story(path: String) {
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
                sleep(100);
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
            sleep(100);
        }
        println!();
    }
}
