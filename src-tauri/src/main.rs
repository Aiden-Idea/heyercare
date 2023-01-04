#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod db;
mod app;
mod utils;

use std::{fs::File, io::{Read, BufReader, BufRead, Seek, SeekFrom}};

use db::db_utils:: init_db ;
use app::{ patients, cmd};

fn main() -> std::io::Result<()> {
    
    // init_db().expect("初始化数据库失败！");

    // let mut file = File::open("src/72710009.Bkp")?;
    // // let mut buffer = [0u8; 8];
    // // println!("{:?}", buffer);
    // // let data = file.read(&mut buffer);
    // // println!("{:?}", buffer);
    // file.seek(SeekFrom::Start(2))?;
    // let mut reader = BufReader::new(file);
    // let mut line = [0u8; 8];
    // // reader.read_line(&mut line);
    // reader.read_exact(&mut line);

    // println!("{:?}", reader);
    // println!("{:?}", line);

    

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            patients::create_patient,
            cmd::run_check_update
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
