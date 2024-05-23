use serde::{Deserialize, Serialize};

pub mod create_task;
pub mod create_task_extractor;
pub mod get_all_tasks;
pub mod get_one_task;

#[derive(Serialize, Deserialize)]
pub struct ResponseTask {
    pub id: i32,
    pub priority: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataTask {
    pub data: ResponseTask,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataTasks {
    pub data: Vec<ResponseTask>,
}
