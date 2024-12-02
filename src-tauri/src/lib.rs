use std::fmt;
use tauri::{AppHandle, ipc::Channel};
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
enum AppEvent<'a> {
  #[serde(rename_all = "camelCase")]
  LoadingStarted {
    id: usize,
  },
  #[serde(rename_all = "camelCase")]
  LoadingProgress {
    id: usize,
    percent: usize,
    message: &'a str,
  },
  #[serde(rename_all = "camelCase")]
  LoadingFinished {
    id: usize,
  },
}

impl<'a> fmt::Display for AppEvent<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppEvent::LoadingStarted { id } => write!(f, "LoadingStarted(id: {})", id),
            AppEvent::LoadingProgress { id, percent, message } => 
                write!(f, "LoadingProgress(id: {}, {}%: {})", id, percent, message),
            AppEvent::LoadingFinished { id } => write!(f, "LoadingFinished(id: {})", id),
        }
    }
}

#[tauri::command]
fn initialize(_app: AppHandle, on_event: Channel<AppEvent>) {
  let id = 1;

  on_event.send(AppEvent::LoadingStarted { id }).unwrap();

  for percent in [10, 30, 75, 90, 100] {
    on_event.send(AppEvent::LoadingProgress { 
      id,
      percent: percent,
      message: "Loading...",
    }).unwrap();
  }

  on_event.send(AppEvent::LoadingFinished { id }).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![initialize])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
