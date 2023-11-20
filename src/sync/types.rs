use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{CourseData, QuestionData, QuestionOptionData, QuestionSourceData, QuestionTopicData};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct SyncData {
    pub courses: ElementSyncData<CourseData, String>,
    pub questions: ElementSyncData<QuestionData, Uuid>,
    pub question_options: ElementSyncData<QuestionOptionData, Uuid>,
    pub question_topics: ElementSyncData<QuestionTopicData, String>,
    pub question_sources: ElementSyncData<QuestionSourceData, String>,
    pub avatar_file_names: HashSet<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ElementSyncData<T: Eq + Hash, K: Eq + Hash> {
    pub for_sync: HashSet<T>,
    pub for_deletion: HashSet<K>,
}

impl<T: Eq + Hash, K: Eq + Hash> Default for ElementSyncData<T, K> {
    fn default() -> Self {
        Self {
            for_sync: Default::default(),
            for_deletion: Default::default(),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncMetadata {
    pub courses: HashMap<String, String>,
    pub questions: HashMap<Uuid, String>,
    pub question_options: HashMap<Uuid, String>,
    pub question_topics: HashSet<String>,
    pub question_sources: HashSet<String>,
}
