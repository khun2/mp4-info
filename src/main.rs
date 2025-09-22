use std::{fs::File, path::PathBuf};

use glob::glob;

struct Info {
    name: String,
    len: String,
}

impl Info {
    fn new() -> Self {
        Self {
            name: String::new(),
            len: String::new(),
        }
    }
}

fn get_all_files() -> glob::Paths {
    glob("./media/*").expect("Failed to read files")
}

fn convert_secs_to_min_and_sec(secs: u64) -> String {
    let mut res = String::new();
    if secs / 60 < 10 {
        res.push('0');
    }
    res = res + &(secs / 60).to_string();
    res.push(':');
    if secs % 60 < 10 {
        res.push('0');
    }
    res = res + &(secs % 60).to_string();
    res
}

fn read_mp4(path: PathBuf) -> Info {
    let f = File::open(path.clone()).unwrap();
    let mp4 = mp4::read_mp4(f).unwrap();
    let mut res = Info::new();
    for track in mp4.tracks().values() {
        res = Info {
            name: String::from(path.to_str().unwrap())
                .split('/')
                .next_back()
                .unwrap()
                .to_string(),
            len: convert_secs_to_min_and_sec(track.duration().as_secs()),
        };
    }
    res
}

fn main() {
    let files = get_all_files();
    let mut output = Vec::new();
    for entry in files {
        match entry {
            Ok(path) => output.push(read_mp4(path)),
            Err(e) => println!("{:?}", e),
        }
    }
    for info in output {
        println!("{}\t{}", info.name, info.len)
    }
}
