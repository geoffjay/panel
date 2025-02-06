use tauri::{ipc::Channel, AppHandle};

use crate::app::event::AppEvent;
use crate::db::establish_connection;
use crate::db::models::{CreateProject, Project};
use crate::repositories::{ProjectRepository, Repository};

#[tauri::command]
pub fn initialize(_app: AppHandle, on_event: Channel<AppEvent>) {
    let id = 1;

    on_event.send(AppEvent::LoadingStarted { id }).unwrap();

    for percent in [10, 30, 75, 90, 100] {
        on_event
            .send(AppEvent::LoadingProgress {
                id,
                percent,
                message: "Loading...",
            })
            .unwrap();
    }

    on_event.send(AppEvent::LoadingFinished { id }).unwrap();
}

#[tauri::command]
pub fn fetch_projects(app: AppHandle) -> Result<Vec<Project>, String> {
    let mut connection = establish_connection(app.clone());
    let projects = match ProjectRepository::all(&mut connection) {
        Ok(projects) => projects,
        Err(e) => return Err(e.to_string()),
    };

    Ok(projects)
}

#[tauri::command]
pub fn create_project(app: AppHandle, project: CreateProject) -> Result<Project, String> {
    let mut connection = establish_connection(app.clone());
    let project = match ProjectRepository::create(&mut connection, project) {
        Ok(project) => project,
        Err(e) => return Err(e.to_string()),
    };

    Ok(project)
}

#[tauri::command]
pub fn delete_project(app: AppHandle, id: i32) -> Result<(), String> {
    let mut connection = establish_connection(app.clone());
    match ProjectRepository::delete(&mut connection, id) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_dashboard_by_project(app: AppHandle, project: Project) -> Result<Project, String> {
    let mut connection = establish_connection(app.clone());
    let project = match ProjectRepository::find(&mut connection, project.id) {
        Ok(project) => project,
        Err(e) => return Err(e.to_string()),
    };

    Ok(project)
}
