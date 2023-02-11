use std::path::Path;
use id3::{Tag, TagLike};
use std::fs;
use colored::Colorize;

fn main() {
    println!("{}", "Hello! This script will try to fix all song file names using their ID3s".yellow());
    let path_str: String = casual::prompt("Enter or paste folder path containing the songs: ").get();
    let path = Path::new(&path_str);

    if !path.exists() {
        println!("{}", "Oh no! The path does not exist!".red());
        return;
    }

    fs::create_dir_all(path.parent().unwrap().join("output")).unwrap();

    for entry in path.read_dir().unwrap() {
        let song_path = entry.unwrap().path();

        let tag = match Tag::read_from_path(&song_path) {
            Ok(tag) => tag,
            Err(_) => {
                match fs::copy(&song_path, &path.parent().unwrap().join("output").join(&song_path.file_name().unwrap())) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("{}", format!("Error moving {} because - {}", &song_path.to_str().unwrap(), e).red());
                    },
                }
                continue;
            },
        };

        let mut rename_filename = String::new();


        if let Some(title) = tag.title() {
            if let Some(artists) = tag.artists() {
                rename_filename.push_str(artists[0]);
                rename_filename.push_str(" - ");
            }
            rename_filename.push_str(title);
        } else {
            rename_filename.push_str(song_path.file_stem().unwrap().to_str().unwrap());
        }
        

        let final_path = &path.parent().unwrap().join("output").join(rename_filename.clone() + ".mp3");
        let final_path_str = final_path.to_str().unwrap().replace("\0", "");
        
        match fs::copy(&song_path, &final_path_str) {
            Ok(_) => (),
            Err(e) => {
                println!("{}", format!("Error moving {} because - {}", &rename_filename, e).red())
            },
        }
    }
}
