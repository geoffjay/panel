use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dashboard {
    pub children: Vec<DashboardElement>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum DashboardElement {
    Stack {
        properties: StackProperties,
        children: Vec<DashboardElement>,
    },
    Panel {
        properties: PanelProperties,
        children: Vec<DashboardElement>,
    },
    Stat {
        properties: StatProperties,
    },
    Range {
        properties: RangeProperties,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LayoutProperties {
    pub margin_top: Option<f32>,
    pub margin_bottom: Option<f32>,
    pub margin_left: Option<f32>,
    pub margin_right: Option<f32>,
    pub padding_top: Option<f32>,
    pub padding_bottom: Option<f32>,
    pub padding_left: Option<f32>,
    pub padding_right: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StackProperties {
    pub direction: Direction,
    #[serde(flatten)]
    pub layout: LayoutProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PanelProperties {
    pub title: String,
    pub subtitle: String,
    #[serde(flatten)]
    pub layout: LayoutProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatProperties {
    pub title: String,
    pub value: String,
    pub description: String,
    #[serde(flatten)]
    pub layout: LayoutProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RangeProperties {
    pub title: String,
    pub value: String,
    pub min: f32,
    pub max: f32,
    #[serde(flatten)]
    pub layout: LayoutProperties,
}
