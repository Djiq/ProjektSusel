use std::net::IpAddr;
use std::ops::Index;
use std::{string, vec};
use std::{
    collections::HashMap,
    sync::RwLock,
};

use async_data_handler::AsyncDataHandler;
use ftp::FtpStream;

mod async_data_handler;
//Imports for databases
//use chrono::{DateTime,Utc};
//use sqlx::{PgPool, postgres::PgQueryResult};

use lazy_static::lazy_static;
use log::log;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! unwrap_or_err {
    ( $e:expr, $err:tt ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return Err($err),
        }
    };
}

macro_rules! unwrap_or_log_and_panic {
    ($e:expr) =>{
        match $e{
            Ok(x) => x,
            Err(err) => {
                log::error!("{}",err);
                panic!();
            }
        }
    }
}

lazy_static! {
    static ref SONGDB: AsyncDataHandler<SongDatabase> = unwrap_or_log_and_panic!(AsyncDataHandler::new("songindex.json"));
    static ref SERVERDB: AsyncDataHandler<ServerDatabase> = unwrap_or_log_and_panic!(AsyncDataHandler::new("serverindex.json"));
    static ref PLAYLISTDB: AsyncDataHandler<PlaylistDatabase> = unwrap_or_log_and_panic!(AsyncDataHandler::new("playlistindex.json"));
    static ref PLAYERSTATE: RwLock<PlayerState> = RwLock::new(PlayerState::default());
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct ServerDatabase{
    servers: Vec<Server>
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct PlaylistDatabase{
    playlists: Vec<Playlist>
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct SongDatabase {
    songs: HashMap<Uuid, Song>,
}

impl SongDatabase {
    fn add_song(&mut self, song: Song) {
        self.songs.insert(song.songid, song);
    }
    
    fn get_song<'a>(&'a self, uuid: &Uuid) -> Option<&'a Song>{
        self.songs.get(uuid)
    }

    fn get_all_songs<'a>(&'a self) -> Vec<&'a Song>{
        self.songs.iter().map(|(k,v)|v).collect()
    }

}

#[derive(Debug, Deserialize, Serialize, Default)]
struct PlayerState{
    current_playlist: Option<usize>
}

impl PlayerState{
    async fn get_next_song(&self) -> Option<Uuid>{
        if self.current_playlist.is_none() {
            None
        } else{
            let playlist_id = self.current_playlist.unwrap();
            let handle  = PLAYLISTDB.get().await;
            let current_song_index_option = handle.playlists[playlist_id].current_song_index;
            if current_song_index_option.is_none() {
                log::error!("Playlist is playing, but it's current song index is undefined - defaulting to 0");
            }
            let index = current_song_index_option.unwrap_or(0);
            Some(handle.playlists[playlist_id].get_songs()[index + 1])
        }
    }

    async fn play(&mut self, song: Uuid){
        
    }
}

impl ServerDatabase{
    fn add_server(&mut self, name: String, ip: String){
        let adress: IpAddr = unwrap_or_err!(
            ip.parse(),
            "Couldn't parse!"
        );
        let serv= Server::new(adress,name);
        match self.servers.binary_search(&serv) {
            Ok(pos) => {} 
            Err(pos) => self.servers.insert(pos, serv),
        }
    }
    fn modify_server(&mut self, old_name: String, name: String, ip: String){
        let adress: IpAddr = unwrap_or_err!(
            ip.parse(),
            "Couldn't parse!"
        );
        let serv= Server::new(adress,name);
        match self.servers.iter().position(|s| s.name == old_name) {
            Some(pos) => self.servers[pos] = serv,
            None => log::error!("No such server found!"),
        }
    }
    fn remove_server(&mut self, name: String, ip: String){
        let adress: IpAddr = unwrap_or_err!(
            ip.parse(),
            "Couldn't parse!"
        );
        let serv= Server::new(adress,name);
        match self.servers.binary_search(&serv) {
            Ok(pos) => self.servers.remove(pos), 
            Err(pos) => {}
        }
    }
}
    

#[derive(Debug, Deserialize, Serialize, Default)]
struct Song {
    songid: Uuid,
    name: String,
    path: String,
    album: Option<String>,
    author: Option<String>,
}

impl Song {
    fn new(songid: Uuid,name: String, path: String, album: Option<String>, author: Option<String>) -> Self {
        Self {
            songid,
            name,
            path,
            album,
            author,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Playlist{
    id: usize,
    name: String,
    desc: Option<String>,
    current_song_index: Option<usize>,
    songs: Vec<Uuid>
}

impl Playlist{
    fn new<A: Into<String>, B: Into<String>>(id: usize,name: A, desc: Option<B>) -> Self{
        Playlist{
            id,
            name: name.into(),
            desc: match desc {
                Some(x) => Some(x.into()),
                None => None,
            },
            current_song_index: None,
            songs: Vec::new()
        }
    }

    fn get_songs(&self) -> Vec<Uuid> {
        self.songs.clone()
    }

    async fn add_song(&mut self, song: Uuid) -> Result<(),&'static str>{
        {
            let songdb_handle = SONGDB.get().await;
            if(songdb_handle.get_song(&song).is_none()){
                return Err("No song with uuid found");
            }
        }
        self.songs.push(song);
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Ord, PartialOrd, PartialEq, Eq)]
struct Server{
    name: String,
    ip: IpAddr,
}

impl Server{
    fn new<A: Into<String>>(ip: IpAddr, name: A)->Self{
        Server{ip, name:name.into()}
    }
}
    

#[tauri::command]
fn ftplist(servername: &str) -> Result<Vec<String>, &str> {
    let mut ftp_stream = unwrap_or_err!(
        FtpStream::connect(servername),
        "Couldn't connect to server!"
    );

    unwrap_or_err!(
        ftp_stream.login("anonymous", "anonymous@example.com"),
        "Couldn't login to server!"
    );

    let file_list = ftp_stream.nlst(None).map_err(|_| "Couldn't list files!")?;
    Ok(file_list)
}

#[tauri::command]
async fn addSong_invoc(name: String, path: String) -> () {
    let song = Song::new(Uuid::new_v4(),name, path, None, None);
    log::info!("Adding song: [{}, {}]",song.name,song.path);
    SONGDB.get_mut().await.add_song(song);
    tokio::spawn(async {SONGDB.save().await});
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Folder {
              path: dirs::data_dir().unwrap().join("boberplayer"),
              file_name: Some("log".to_owned()),
            },
          )).build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![addSong_invoc, ftplist])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
