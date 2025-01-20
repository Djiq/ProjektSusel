use lazy_static::lazy_static;
use crate::async_data_handler::AsyncDataHandler;
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use uuid::Uuid;
use ftp::FtpStream;
use std::collections::HashMap;
use std::net::IpAddr;
use std::ops::Index;

lazy_static! {
    pub static ref SONGDB: AsyncDataHandler<SongDatabase> = unwrap_or_log_and_panic!(AsyncDataHandler::new("songindex.json"));
    pub static ref SERVERDB: AsyncDataHandler<ServerDatabase> = unwrap_or_log_and_panic!(AsyncDataHandler::new("serverindex.json"));
    pub static ref PLAYLISTDB: AsyncDataHandler<PlaylistDatabase> = unwrap_or_log_and_panic!(AsyncDataHandler::new("playlistindex.json"));
    pub static ref PLAYERSTATE: RwLock<PlayerState> = RwLock::new(PlayerState::default());
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ServerDatabase{
    pub servers: Vec<Server>
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PlaylistDatabase{
    pub playlists: Vec<Playlist>
}

impl PlaylistDatabase{
    pub fn add_playlist(&mut self,name: String, desc: Option<String>){
        self.playlists.push(Playlist::new(self.playlists.len(), name, desc));
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SongDatabase {
    pub songs: HashMap<Uuid, Song>,
}

impl SongDatabase {
    pub fn add_song(&mut self, song: Song) {
        self.songs.insert(song.songid, song);
    }
    
    pub fn get_song<'a>(&'a self, uuid: &Uuid) -> Option<&'a Song>{
        self.songs.get(uuid)
    }

    pub fn get_all_songs<'a>(&'a self) -> Vec<&'a Song>{
        self.songs.iter().map(|(k,v)|v).collect()
    }

}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PlayerState{
    pub current_playlist: Option<usize>
}

impl PlayerState{
    pub async fn get_next_song(&self) -> Option<Uuid>{
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

    pub async fn play(&mut self, song: Uuid){
        
    }
}

impl ServerDatabase{
    pub fn add_server(&mut self, name: String, ip: String) -> Result<(), std::net::AddrParseError>{
        let adress:IpAddr = match ip.parse::<IpAddr>(){
            Ok(result) => result,
            Err(err) => return Err(err),
        };
        let serv= Server::new(adress,name);
        match self.servers.binary_search(&serv) {
            Ok(_) => {} 
            Err(pos) => self.servers.insert(pos, serv),
        }
        Ok(())
    }
    pub fn modify_server(&mut self, old_name: String, name: String, ip: String) -> Result<(), std::net::AddrParseError>{
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
    pub fn remove_server(&mut self, name: String) {
        match self.servers.binary_search_by_key(&name, |s| s.name.clone()) {
            Ok(pos) => {
                self.servers.remove(pos);
            }
            Err(_) => log::error!("No such server found!"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default,Clone)]
pub struct Song {
    pub songid: Uuid,
    pub name: String,
    pub path: String,
    pub album: Option<String>,
    pub author: Option<String>,
}

impl Song {
    pub fn new(songid: Uuid,name: String, path: String, album: Option<String>, author: Option<String>) -> Self {
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
pub struct Playlist{
    pub id: usize,
    pub name: String,
    pub desc: Option<String>,
    pub current_song_index: Option<usize>,
    pub songs: Vec<Uuid>
}

impl Playlist{
    pub fn new<A: Into<String>, B: Into<String>>(id: usize,name: A, desc: Option<B>) -> Self{
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

    pub fn get_songs<'a>(&'a self) -> &'a Vec<Uuid> {
        &self.songs
    }

    pub async fn add_song(&mut self, song: Uuid) -> Result<(),&'static str>{
        {
            let songdb_handle = SONGDB.get().await;
            if songdb_handle.get_song(&song).is_none() {
                return Err("No song with uuid found");
            }
        }
        self.songs.push(song);
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct Server{
    pub name: String,
    pub ip: IpAddr,
}

impl Server{
    pub fn new<A: Into<String>>(ip: IpAddr, name: A)->Self{
        Server{ip, name:name.into()}
    }
}