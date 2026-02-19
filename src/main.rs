// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod models;
mod services;
mod commands;
mod state;

use std::sync::Mutex;
use state::AppState;

/* ===========================
   AUTH COMMANDS
=========================== */

use commands::auth::{
    register_user,
    login,
    get_current_user,
    logout_user,
    update_user_profile,
    upload_avatar,
};

/* ===========================
   PROJECT COMMANDS
=========================== */

use commands::projects::{
    get_projects,
    create_project,
    update_project,
    delete_project,
};

/* ===========================
   SESSION COMMANDS
=========================== */

use commands::sessions::{
    get_active_session,
    start_session,
    finalize_session,
    cancel_session,
    get_finished_sessions,
};

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            current_user_id: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            // AUTH
            register_user,
            login,
            get_current_user,
            logout_user,
            update_user_profile,
            upload_avatar,

            // PROJECTS
            get_projects,
            create_project,
            update_project,
            delete_project,

            // SESSIONS
            get_active_session,
            start_session,
            finalize_session,
            cancel_session,
            get_finished_sessions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}