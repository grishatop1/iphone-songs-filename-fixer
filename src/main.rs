use std::path::Path;
use id3::{Tag, TagLike};
use std::fs;

fn main() {
    println!("Hello! This script will try to fix all song file names using their ID3s");
    let path_str: String = casual::prompt("Enter or paste folder path containing the songs: ").get();
    let path = Path::new(&path_str);

    if !path.exists() {
        println!("Oh no! The path does not exist!");
        return;
    }

    fs::create_dir_all(path.parent().unwrap().join("output")).unwrap();

    for entry in path.read_dir().unwrap() {
        let song_path = entry.unwrap().path();

        let tag = match Tag::read_from_path(&song_path) {
            Ok(tag) => tag,
            Err(_) => continue,
        };

        let mut rename_filename = String::new();


        if let Some(title) = tag.title() {
            if let Some(artist) = tag.artist() {
                rename_filename.push_str(artist);
                rename_filename.push_str(" - ");
            }
            rename_filename.push_str(title);
        } else {
            rename_filename.push_str(song_path.file_stem().unwrap().to_str().unwrap());
        }

        match fs::copy(&song_path, &path.parent().unwrap().parent().unwrap().join("output").join(rename_filename).join(".mp3")) {
            Ok(_) => (),
            Err(_) => (),
        }
    }

}
