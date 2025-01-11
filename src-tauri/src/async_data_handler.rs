use std::{fs::{self, File}, io::{BufWriter, Read, Write}, ops::Deref, path::PathBuf};
use std::default;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub fn get_config_file<T: Into<String>>(filename: T) -> Result<File,&'static str>{
    let filename_string = filename.into();
    log::info!("Acquiring file handler for {}",&filename_string);

    let mut data_dir = match dirs::data_dir() {
        Some(x) => x,
        None => return Err("Couldn't access data directory!"),
    };

    data_dir = data_dir.join("boberplayer");
    if !data_dir.exists() {
        if fs::create_dir(data_dir.clone()).is_err(){
            return Err("Couldn't create boberplayer data directory!")
        }
    }
    let data_file = data_dir.join(filename_string);

    let open_file: File = match File::options().read(true).write(true).open(data_file){
        Ok(x) => x,
        Err(_) => return Err("Couldn't open file"),
    };
        
    Ok(open_file)
}

#[derive(Debug)]
pub struct AsyncDataHandler<T> where T: Serialize, T: DeserializeOwned, T:Default, T:Send{
    file: Mutex<File>,
    data: RwLock<T>
}

impl<'a,T> AsyncDataHandler<T> where T: Serialize, T: DeserializeOwned, T: Default, T:Send{

    pub fn new<A:Into<String>>(filename: A) -> Result<Self,&'static str>{  
        let mut file = get_config_file(filename)?;
        let mut data = String::new();
        match file.read_to_string(&mut data){
            Ok(_) => {},
            Err(_) => return Err("Couldn't parse file into string!")
        };  

        let data: T = match serde_json::from_str(&data){
            Ok(a) => a,
            Err(_) => T::default()
        };

        Ok(AsyncDataHandler{
            file: Mutex::new(file),
            data: RwLock::new(data)
        })
    }

    pub async fn save(&self) -> Result<(),&'static str>{
        
        let serialized = {
            let data_reader = self.get().await;
            match serde_json::to_string(&data_reader.deref()){
                Ok(string) => string,
                Err(_) => return Err("Couldn't serialize the data")
            }
            
        };
        match self.file.lock().await.write_all(serialized.as_bytes()){
            Ok(_) => Ok(()),
            Err(_) => Err("Couldn't write data to file!")
        }
        
    }

    pub async fn get(&self) -> RwLockReadGuard<T>{
        return self.data.read().await;
    }

    pub async fn get_mut(&self) ->  RwLockWriteGuard<T>{
        return self.data.write().await;
    }

}
