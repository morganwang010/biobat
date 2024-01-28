// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[macro_use]
extern crate diesel;
use tokio;
mod db;
mod cmd;
mod info;
mod models;
mod schema;
mod server;
use tauri::{GlobalWindowEvent, Manager, WindowEvent, Wry};

fn handle_window_event(event: GlobalWindowEvent<Wry>) {
  let window = event.window();
  // let app = window.app_handle();

  match event.event() {
    WindowEvent::CloseRequested { api, .. } => {
      let window = window.clone();

      window.hide().unwrap();
      api.prevent_close();
    }
    _ => {}
  }
}

#[tokio::main]  
async fn main() {
  let context = tauri::generate_context!();

  let mut connection = db::establish_connection();
  let window = tauri::Builder::default();

  window
  .on_window_event(handle_window_event)
  .invoke_handler(tauri::generate_handler![
    cmd::fetch_ba,
  ])
    .build(context)
    .expect("error while running tauri application")
    .run(|_app_handle, event| match event {
      tauri::RunEvent::ExitRequested { api, .. } => {
        api.prevent_exit();
      }
      _ => {}
    });
}
