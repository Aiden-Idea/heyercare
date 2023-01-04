use crate::{
  utils::{self}
};
use tauri::{ command, AppHandle };

#[command]
pub fn run_check_update(app: AppHandle) -> () {
  println!("app_update");
  utils::run_check_update(app).unwrap();
}