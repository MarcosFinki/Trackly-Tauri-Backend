use tauri::{command, State};
use serde::Deserialize;

use crate::state::AppState;
use crate::services::project_service;

#[derive(Deserialize)]
pub struct CreateProjectInput {
    pub name: String,
    pub color: String,
}

#[derive(Deserialize)]
pub struct UpdateProjectInput {
    pub id: i64,
    pub name: Option<String>,
    pub color: Option<String>,
}

#[command]
pub fn get_projects(
    state: State<AppState>,
) -> Result<Vec<crate::models::project::Project>, String> {
    let user_id = state
        .current_user_id
        .lock()
        .unwrap()
        .ok_or("Not authenticated")?;

    project_service::get_projects(user_id)
}

#[command]
pub fn create_project(
    state: State<AppState>,
    input: CreateProjectInput,
) -> Result<crate::models::project::Project, String> {
    let user_id = state
        .current_user_id
        .lock()
        .unwrap()
        .ok_or("Not authenticated")?;

    project_service::create_project(user_id, &input.name, &input.color)
}

#[command]
pub fn update_project(
    state: State<AppState>,
    input: UpdateProjectInput,
) -> Result<(), String> {
    let user_id = state
        .current_user_id
        .lock()
        .unwrap()
        .ok_or("Not authenticated")?;

    project_service::update_project(user_id, input.id, input.name, input.color)
}

#[command]
pub fn delete_project(
    state: State<AppState>,
    id: i64,
) -> Result<(), String> {
    let user_id = state
        .current_user_id
        .lock()
        .unwrap()
        .ok_or("Not authenticated")?;

    project_service::delete_project(user_id, id)
}