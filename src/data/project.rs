use serde::{Deserialize, Serialize};
use serde_yaml_ng as serde_yaml;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub notes: Vec<Note>,
    pub created_by: String,
    pub created_at: u64,
    pub last_updated: u64,
    pub project_owner: String,
    pub project_type: ProjectType,
    pub history: Vec<ProjectHistory>,
    pub data_location: String,
    pub data_size: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub body: String,
    pub created_by: String,
    pub created_at: u64,
    pub last_updated: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProjectType {
    DNA(String),
    RNA(String),
}

impl Default for ProjectType {
    fn default() -> Self {
        ProjectType::DNA("".to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectHistory {
    pub action: ProjectAction,
    pub created_by: String,
    pub created_at: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProjectAction {
    Basecall(BasecallLog),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasecallLog {
    pub status: BasecallStatus,
    pub basecaller: String,
    pub version: String,
    pub config: String,
    pub basecall_run: BasecallRun,
    pub command: Vec<String>,
    pub results: Option<BasecallResults>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BasecallStatus {
    Success,
    Failed,
    InProgress,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasecallRun {
    pub name: String,
    pub read_count: u64,
    pub output_path: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasecallResults {
    pub read_count: u64,
    pub mean_qscore: f64,
    pub median_qscore: f64,
    pub min_qscore: f64,
    pub max_qscore: f64,
    pub n50: u64,
    pub n_bases: u64,
}
