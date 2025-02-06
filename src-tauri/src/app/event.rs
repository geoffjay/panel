use serde::Serialize;
use std::fmt;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
#[allow(clippy::enum_variant_names)]
pub enum AppEvent<'a> {
    #[serde(rename_all = "camelCase")]
    LoadingStarted { id: usize },
    #[serde(rename_all = "camelCase")]
    LoadingProgress {
        id: usize,
        percent: usize,
        message: &'a str,
    },
    #[serde(rename_all = "camelCase")]
    LoadingFinished { id: usize },
}

impl fmt::Display for AppEvent<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppEvent::LoadingStarted { id } => write!(f, "LoadingStarted(id: {})", id),
            AppEvent::LoadingProgress {
                id,
                percent,
                message,
            } => write!(f, "LoadingProgress(id: {}, {}%: {})", id, percent, message),
            AppEvent::LoadingFinished { id } => write!(f, "LoadingFinished(id: {})", id),
        }
    }
}
