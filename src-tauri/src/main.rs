// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate diesel;
extern crate diesel_migrations;
use dotenv::dotenv;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::LevelFilter;
use tauri::{GlobalWindowEvent, Manager, WindowEvent, Wry};
use tauri_plugin_log::{fern, LogTarget};
use tokio;

mod cmd;
mod db;
mod info;
mod models;
mod schema;
mod server;
use std::env;

// pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

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
    dotenv().ok();

    let _env = env::var("BIOINFO_ENV");

    let context = tauri::generate_context!();

    let mut connection = db::establish_connection();

    // connection
    // .run_pending_migrations(MIGRATIONS)
    // .expect("Error migrating");

    let window = tauri::Builder::default();

    window
        .on_window_event(handle_window_event)
        .plugin(
            tauri_plugin_log::Builder::default()
              .targets([LogTarget::LogDir, LogTarget::Stdout])
              .with_colors(fern::colors::ColoredLevelConfig::default())
              .level(
                env::var("BIOINFO_ENV")
                  .map(|_env| LevelFilter::Debug)
                  .unwrap_or(LevelFilter::Info),
              )
              .build(),
          )
        .setup(|app| {
            let app_handle = app.handle();

            let main_window = app.get_window("main").unwrap();

            match env::var("LETTURA_ENV") {
                Ok(_env) => {
                    // main_window.set_title("Lettura dev").unwrap();
                }
                Err(_) => {}
            }
            let boxed_handle = Box::new(app_handle);

            std::thread::spawn(move || {
                server::init(*boxed_handle).unwrap();
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![cmd::fetch_ba,])
        .build(context)
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
