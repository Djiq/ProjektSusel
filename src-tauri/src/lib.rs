use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    hash::Hash,
    io::{BufWriter,Read, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};
use std::collections;


//Imports for databases
//use chrono::{DateTime,Utc};
//use sqlx::{PgPool, postgres::PgQueryResult};

use dirs::data_dir;
use serde::{Deserialize, Serialize};
use tauri::State;
use tauri_plugin_shell::open;

macro_rules! unwrap_or_err {
    ( $e:expr, $err:tt ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return Err($err),
        }
    };
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Song {
    name: String,
    path: String,
    album: Option<String>,
    author: Option<String>,
}

impl Song{
    fn new(name: String, path: String, album: Option<String>, author: Option<String>) -> Self{
        Self { name, path, album, author }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct SongDatabase {
    songs: HashMap<String, Song>,
}

impl SongDatabase {
    fn new() -> Result<Self, &'static str> {
        let mut db = SongDatabase {
            songs: HashMap::new(),
        };

        let mut data_dir = match dirs::data_dir() {
            Some(x) => x,
            None => return Err("Couldn't access data directory!"),
        };

        data_dir = data_dir.join("boberplayer");
        if !data_dir.exists() {
            unwrap_or_err!(
                fs::create_dir(data_dir.clone()),
                "Couldn't create boberplayer data directory!"
            );
        }

        let data_file = data_dir.join("songindex.json");

        let mut open_file: File = unwrap_or_err!(File::options().read(true).write(false).open(data_file), "Couldn't open boberplayer music index file!");

        let mut data = String::new();
        unwrap_or_err!(
            open_file.read_to_string(&mut data),
            "Couldn't read data from boberplayer music index file!"
        );

        db.songs = unwrap_or_err!(
            serde_json::from_str::<HashMap<String, Song>>(&data),
            "Couldn't deserialize boberplayer music index file!"
        );


        Ok(db)
    }

    fn addSong(&mut self, song: Song) -> Result<(), &'static str> {
        let mut data_dir = match dirs::data_dir() {
            Some(x) => x,
            None => return Err("Couldn't access data directory!"),
        };

        data_dir = data_dir.join("boberplayer");
        if !data_dir.exists() {
            unwrap_or_err!(
                fs::create_dir(data_dir.clone()),
                "Couldn't create boberplayer data directory!"
            );
        }

        let data_file = data_dir.join("songindex.json");

        let mut open_file: File =  unwrap_or_err!(File::options().read(false).write(true).open(data_file), "Couldn't open boberplayer music index file!");
        self.songs.insert(song.path.to_owned(), song);

        let json = unwrap_or_err!(
            serde_json::to_string(&self.songs),
            "Couldn't serialize song data into music index file!"
        );

        println!("{}",json);

        let mut writer = BufWriter::new(open_file);

        unwrap_or_err!(
            serde_json::to_writer(&mut writer, &self.songs),
            "Writing data to music index file failed!"
        );

        match writer.flush(){
            Ok(x)=>println!("Buffer flushed! {:?}",x),
            Err(x)=>println!("Flushing failed {}", x)
        }
        unwrap_or_err!(
            writer.flush(),
            "Buffer flushing failed!"
        );

        Ok(())
    }
}

// #[derive(Debug)]
// struct ServerData {
//     server: String,
//     server_id: i32,
//     last_update: DateTime<Utc>,
// }



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn addSong_invoc(name: String, path: String) -> () {
    let mut db = SongDatabase::new().unwrap();
    let song = Song::new(name, path, None, None);
    match db.addSong(song){
        Ok(x)=>println!("Wrote properly! {:?}",x),
        Err(x)=>println!("Wrote improperly! {}",x)
    };
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet,addSong_invoc])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
