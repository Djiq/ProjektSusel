
use std::vec;


mod async_data_handler;
mod commands;
#[macro_use]
mod macros;
mod statics;

use crate::statics::*;
use crate::commands::*;

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
        .invoke_handler(tauri::generate_handler![cmd_add_song, cmd_ftplist,cmd_get_all_songs,cmd_get_playlist_songs,cmd_get_playlists,cmd_add_playlist,cmd_add_song_to_playlist, cmd_add_server, cmd_rm_server, cmd_mod_server, cmd_get_servers, cmd_get_playlist,cmd_get_song,cmd_download])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
