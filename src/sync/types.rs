use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{CourseData, QuestionData, QuestionOptionData, QuestionSourceData, QuestionTopicData};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct SyncData {
    pub courses: CoursesSyncData,
    pub questions: QuestionsSyncData,
    pub question_options: QuestionOptionsSyncData,
    pub question_topics: QuestionTopicsSyncData,
    pub question_sources: QuestionSourcesSyncData,
    pub avatar_file_names: HashSet<String>,
}

pub type CoursesSyncData = ElementSyncData<CourseData, String>;
pub type QuestionsSyncData = ElementSyncData<QuestionData, Uuid>;
pub type QuestionOptionsSyncData = ElementSyncData<QuestionOptionData, Uuid>;
pub type QuestionTopicsSyncData = ElementSyncData<QuestionTopicData, String>;
pub type QuestionSourcesSyncData = ElementSyncData<QuestionSourceData, String>;

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
