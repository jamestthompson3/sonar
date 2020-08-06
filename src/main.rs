use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use walkdir::{DirEntry, WalkDir};

fn is_mp3(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| {
            println!("{}", s);
            s
        })
        .map(|s| s.ends_with("mp3"))
        .unwrap_or(false)
}

fn main() {
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);

    let walker = WalkDir::new("/home/taylor/Music").into_iter();
    for entry in walker {
        let entry = entry.unwrap();
        let fname = entry.file_name().to_str().unwrap();
        if fname.ends_with("mp3") {
            println!("Now Playing: \x1b[38;5;81m{}\x1b[0m", fname);
            let file = std::fs::File::open(entry.path()).unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            sink.append(source);
            sink.sleep_until_end();
        }
    }
}
