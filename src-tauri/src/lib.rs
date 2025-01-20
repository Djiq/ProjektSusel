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
mod commands;
#[macro_use]
mod macros;


use lazy_static::lazy_static;
use log::log;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub(crate) use unwrap_or_err;
pub(crate) use unwrap_or_log_and_panic;

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

impl PlaylistDatabase{
    fn add_playlist(&mut self,name: String, desc: Option<String>){
        self.playlists.push(Playlist::new(self.playlists.len(), name, desc));
    }
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
    fn add_server(&mut self, name: String, ip: String) -> Result<(), std::net::AddrParseError>{
        let adress:IpAddr = match ip.parse::<IpAddr>(){
            Ok(result) => result,
            Err(err) => return Err(err),
        };
        let serv= Server::new(adress,name);
        match self.servers.binary_search(&serv) {
            Ok(pos) => {} 
            Err(pos) => self.servers.insert(pos, serv),
        }
        Ok(())
    }
    fn modify_server(&mut self, old_name: String, name: String, ip: String) -> Result<(), std::net::AddrParseError>{
        let adress:IpAddr = match ip.parse::<IpAddr>(){
            Ok(result) => result,
            Err(err) => return Err(err),
        };
        let serv= Server::new(adress,name);
        match self.servers.binary_search_by_key(&old_name,|s| s.name.clone()) {
            Ok(pos) => self.servers[pos] = serv,
            Err(_) => log::error!("No such server found!"),
        }
        Ok(())
    }
    fn remove_server(&mut self, name: String) {
        match self.servers.binary_search_by_key(&name, |s| s.name.clone()) {
            Ok(pos) => {
                self.servers.remove(pos);
            }
            Err(_) => log::error!("No such server found!"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default,Clone)]
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

#[derive(Debug, Deserialize, Serialize, Default,Clone)]
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

    fn get_songs<'a>(&'a self) -> &'a Vec<Uuid> {
        &self.songs
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

#[derive(Debug, Deserialize, Serialize, Ord, PartialOrd, PartialEq, Eq, Clone)]
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
async fn cmd_get_servers() -> Vec<Server>{
    SERVERDB.get().await.servers.to_vec()
}

#[tauri::command]
async fn cmd_add_server(name: String, ip: String) -> (){
    match SERVERDB.get_mut().await.add_server(name, ip){
        Ok(_) => {},
        Err(err) => log::error!("Failed to add server! Reason:{}",err)
    };
    tokio::spawn(async {SERVERDB.save().await});
}
#[tauri::command]
async fn cmd_rm_server(name: String){
    SERVERDB.get_mut().await.remove_server(name);
    tokio::spawn(async {SERVERDB.save().await});
}
#[tauri::command]
async fn cmd_mod_server(oldname:String, newname: String, ip:String){
    match SERVERDB.get_mut().await.modify_server(oldname, newname, ip){
        Ok(_) => {},
        Err(err) => log::error!("Failed to modify server! Reason:{}",err)
    };
    tokio::spawn(async {SERVERDB.save().await});
}
    

#[tauri::command]
fn cmd_ftplist(servername: &str) -> Result<Vec<String>, &str> {
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
async fn cmd_add_song(path: String,name: String,album: Option<String>,author: Option<String>) -> () {
    let song = Song::new(Uuid::new_v4(),name, path, album, author);
    log::info!("Adding song: [{}, {}]",song.name,song.path);
    SONGDB.get_mut().await.add_song(song);
    tokio::spawn(async {unwrap_or_log_and_panic!(SONGDB.save().await)});
}

#[tauri::command]
async fn cmd_get_all_songs() -> Vec<Song> {
    SONGDB.get().await.get_all_songs().iter().map(|song| (*song).clone()).collect()
}

#[tauri::command]
async fn cmd_get_playlist_songs(playlist: usize) -> Vec<Song>{
    let uuid_vec = PLAYLISTDB.get().await.playlists[playlist].get_songs().clone();
    let songdb = SONGDB.get().await;
    uuid_vec.iter().map(|uuid|songdb.get_song(uuid).unwrap().clone()).collect()
}

#[tauri::command]
async fn cmd_get_playlists() -> Vec<Playlist> {
    PLAYLISTDB.get().await.playlists.clone()
}

#[tauri::command]
async fn cmd_add_playlist(name: String, desc: Option<String>) {
    PLAYLISTDB.get_mut().await.add_playlist(name, desc);
    tokio::spawn(async {unwrap_or_log_and_panic!(PLAYLISTDB.save().await)});
}

#[tauri::command]
async fn cmd_add_song_to_playlist(song: Uuid, playlist: usize){
    unwrap_or_log_and_panic!(PLAYLISTDB.get_mut().await.playlists[playlist].add_song(song).await);
}

#[tauri::command]
async fn cmd_get_song(song: Uuid) -> Song{
    unwrap_or_log_and_panic!(SONGDB.get().await.get_song(song))
}

#[tauri::command]
async fn cmd_get_playlist(playlist: usize) -> Song{
    unwrap_or_log_and_panic!(PLAYLISTDB.get().await.playlists.get(playlist))
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
        .invoke_handler(tauri::generate_handler![cmd_add_song, cmd_ftplist,cmd_get_all_songs,cmd_get_playlist_songs,cmd_get_playlists,cmd_add_playlist,cmd_add_song_to_playlist, cmd_add_server, cmd_rm_server, cmd_mod_server, cmd_get_servers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
