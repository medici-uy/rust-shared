use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{CourseData, QuestionData, QuestionOptionData, QuestionSourceData};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct SyncData {
    pub courses: CoursesSyncData,
    pub questions: QuestionsSyncData,
    pub question_options: QuestionOptionsSyncData,
    pub question_sources: QuestionSourcesSyncData,
    pub avatar_file_names: HashSet<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct CoursesSyncData {
    pub for_sync: HashSet<CourseData>,
    pub for_deletion: HashSet<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct QuestionsSyncData {
    pub for_sync: HashSet<QuestionData>,
    pub for_deletion: HashSet<Uuid>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct QuestionOptionsSyncData {
    pub for_sync: HashSet<QuestionOptionData>,
    pub for_deletion: HashSet<Uuid>,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct QuestionSourcesSyncData {
    pub for_sync: HashSet<QuestionSourceData>,
    pub for_deletion: HashSet<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncMetadata {
    pub courses: HashMap<String, String>,
    pub questions: HashMap<Uuid, String>,
    pub question_options: HashMap<Uuid, String>,
    pub question_sources: HashSet<String>,
}
