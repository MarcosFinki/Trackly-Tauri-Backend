use tauri::{command, State};

use crate::state::AppState;
use crate::services::session_service;
use crate::models::session::{
    ActiveSessionResponse,
    FinishedSessionResponse,
};

/* ===========================
   GET ACTIVE SESSION
=========================== */

#[command]
pub fn get_active_session(
    state: State<AppState>,
) -> Result<Option<ActiveSessionResponse>, String> {

    let current = state.current_user_id.lock().unwrap();
    let user_id = current.ok_or("Not authenticated")?;

    session_service::get_active_session(user_id)
}

/* ===========================
   START SESSION
=========================== */

#[command]
pub fn start_session(
    state: State<AppState>,
    project_id: Option<i64>,
) -> Result<ActiveSessionResponse, String> {

    let current = state.current_user_id.lock().unwrap();
    let user_id = current.ok_or("Not authenticated")?;

    session_service::start_session(user_id, project_id)
}

/* ===========================
   FINALIZE SESSION
=========================== */

#[command]
pub fn finalize_session(
    state: State<AppState>,
    session_id: i64,
    description: String,
    tags: Vec<String>,
) -> Result<(), String> {

    let current = state.current_user_id.lock().unwrap();
    let user_id = current.ok_or("Not authenticated")?;

    println!("FINALIZE called");
    println!("user_id: {}", user_id);
    println!("session_id: {}", session_id);

    session_service::finalize_session(
        user_id,
        session_id,
        description,
        tags,
    )?;

    println!("FINALIZE SUCCESS");

    Ok(())
}

/* ===========================
   CANCEL SESSION
=========================== */

#[command]
pub fn cancel_session(
    state: State<AppState>,
) -> Result<(), String> {

    let current = state.current_user_id.lock().unwrap();
    let user_id = current.ok_or("Not authenticated")?;

    session_service::cancel_session(user_id)?;

    Ok(())
}

/* ===========================
   FINISHED SESSIONS
=========================== */

#[command]
pub fn get_finished_sessions(
    state: State<AppState>,
) -> Result<Vec<FinishedSessionResponse>, String> {

    let current = state.current_user_id.lock().unwrap();
    let user_id = current.ok_or("Not authenticated")?;

    session_service::get_finished_sessions(user_id)
}