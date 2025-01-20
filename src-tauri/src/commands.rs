
use crate::*;

use uuid::Uuid;
use ftp::FtpStream;


#[tauri::command]
pub fn cmd_ftplist(servername: &str) -> Result<Vec<String>, &str> {
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
pub async fn cmd_add_song(path: String,name: String,album: Option<String>,author: Option<String>) -> () {
    let song = Song::new(Uuid::new_v4(),name, path, album, author);
    log::info!("Adding song: [{}, {}]",song.name,song.path);
    SONGDB.get_mut().await.add_song(song);
    tokio::spawn(async {unwrap_or_log_and_panic!(SONGDB.save().await)});
}

#[tauri::command]
pub async fn cmd_get_all_songs() -> Vec<Song> {
    SONGDB.get().await.get_all_songs().iter().map(|song| (*song).clone()).collect()
}

#[tauri::command]
pub async fn cmd_get_playlist_songs(playlist: usize) -> Vec<Song>{
    let uuid_vec = PLAYLISTDB.get().await.playlists[playlist].get_songs().clone();
    let songdb = SONGDB.get().await;
    uuid_vec.iter().map(|uuid|songdb.get_song(uuid).unwrap().clone()).collect()
}

#[tauri::command]
pub async fn cmd_get_playlists() -> Vec<Playlist> {
    PLAYLISTDB.get().await.playlists.clone()
}

#[tauri::command]
pub async fn cmd_add_playlist(name: String, desc: Option<String>) {
    PLAYLISTDB.get_mut().await.add_playlist(name, desc);
    tokio::spawn(async {unwrap_or_log_and_panic!(PLAYLISTDB.save().await)});
}

#[tauri::command]
pub async fn cmd_add_song_to_playlist(song: Uuid, playlist: usize){
    unwrap_or_log_and_panic!(PLAYLISTDB.get_mut().await.playlists[playlist].add_song(song).await);
}

#[tauri::command]
pub async fn cmd_get_song(song: Uuid) -> Song{
    unwrap_or_log_and_panic_option!(SONGDB.get().await.get_song(&song),"No song found!").clone()
}

#[tauri::command]
pub async fn cmd_get_playlist(playlist: usize) -> Playlist{
    unwrap_or_log_and_panic_option!(PLAYLISTDB.get().await.playlists.get(playlist),"No playlist found").clone()
}

#[tauri::command]
pub async fn cmd_get_servers() -> Vec<Server>{
    SERVERDB.get().await.servers.to_vec()
}

#[tauri::command]
pub async fn cmd_add_server(name: String, ip: String) -> (){
    match SERVERDB.get_mut().await.add_server(name, ip){
        Ok(_) => {},
        Err(err) => log::error!("Failed to add server! Reason:{}",err)
    };
    tokio::spawn(async {SERVERDB.save().await});
}
#[tauri::command]
pub async fn cmd_rm_server(name: String){
    SERVERDB.get_mut().await.remove_server(name);
    tokio::spawn(async {SERVERDB.save().await});
}
#[tauri::command]
pub async fn cmd_mod_server(oldname:String, newname: String, ip:String){
    match SERVERDB.get_mut().await.modify_server(oldname, newname, ip){
        Ok(_) => {},
        Err(err) => log::error!("Failed to modify server! Reason:{}",err)
    };
    tokio::spawn(async {SERVERDB.save().await});
}
    