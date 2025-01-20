use std::{fs::{self, File}, io::{self, Read, Seek, SeekFrom, Write}, ops::Deref};

use log::kv::Error;
use serde::{de::DeserializeOwned, Serialize};
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

    let data_file = data_dir.join(&filename_string);

    let open_file: File = match fs::exists(&data_file) {
        Ok(false)=> match File::create(&data_file) {
            Ok(x) => x,
            Err(_) => return Err("Couldn't create file"),
        }
        Ok(true) => match File::options().read(true).write(true).open(&data_file){
            Ok(x) => x,
            Err(_) => return Err("Couldn't open file"),
        }
        Err(_) => return Err("Error during filesystem resolution - file cannot be determined to exist"),
    };

    log::info!("Successfully acquired file handle for {}", filename_string);

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
        let data = match file.read_to_string(&mut data){
            Ok(_) => {
                match serde_json::from_str(&data){
                    Ok(a) => a,
                    Err(_) => {
                        log::error!("Defaulting - Couldn't parse string into data");
                        T::default()
                    }
                }
            },
            Err(_) => {
                log::error!("Defaulting - Couldn't parse file into string");
                T::default()
            }
        };   

        Ok(AsyncDataHandler{
            file: Mutex::new(file),
            data: RwLock::new(data)
        })
    }

    fn seek_write(file: &mut tokio::sync::MutexGuard<'_, std::fs::File, >, data: &[u8], offset: u64) -> Result<(), io::Error> {
        file.seek(SeekFrom::Start(offset))?;
        file.write_all(data)?;
        Ok(())
    }

    pub async fn save(&self) -> Result<(),&'static str>{
        
        let serialized = {
            let data_reader = self.get().await;
            match serde_json::to_string(&data_reader.deref()){
                Ok(string) => string,
                Err(_) => return Err("Couldn't serialize the data")
            }
            
        };
        
        match Self::seek_write(&mut self.file.lock().await, serialized.as_bytes(), 0){
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