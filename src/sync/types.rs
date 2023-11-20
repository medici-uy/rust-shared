use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{CourseData, QuestionData, QuestionOptionData, QuestionSourceData};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncData {
    pub courses_to_sync: Vec<CourseData>,
    pub courses_to_delete: Vec<String>,

    pub questions_to_sync: Vec<QuestionData>,
    pub questions_to_delete: Vec<Uuid>,

    pub question_options_to_sync: Vec<QuestionOptionData>,
    pub question_options_to_delete: Vec<Uuid>,

    pub course_evaluations_to_sync: Vec<QuestionSourceData>,
    pub course_evaluations_to_delete: Vec<String>,

    pub avatar_file_names: HashSet<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncMetadata {
    pub courses_metadata: HashMap<String, String>,
    pub questions_metadata: HashMap<Uuid, String>,
    pub question_options_metadata: HashMap<Uuid, String>,
    pub course_evaluations_metadata: HashMap<String, String>,
}
