use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    BundleData, CourseData, IconData, QuestionData, QuestionOptionData, QuestionSourceData,
    QuestionTopicData,
};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct SyncData {
    pub courses: CoursesSyncData,
    pub questions: QuestionsSyncData,
    pub question_options: QuestionOptionsSyncData,
    pub question_topics: QuestionTopicsSyncData,
    pub question_sources: QuestionSourcesSyncData,
    pub bundles: BundlesSyncData,
    pub icons: IconsSyncData,
}

impl Display for SyncData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "Courses: {}\nQuestions: {}\nQuestion options: {}\nQuestion topics: {}\nQuestion sources: {}\nBundles: {}\nIcons: {}",
            self.courses,
            self.questions,
            self.question_options,
            self.question_topics,
            self.question_sources,
            self.bundles,
            self.icons
        )
    }
}

pub type CoursesSyncData = ElementSyncData<CourseData, String>;
pub type QuestionsSyncData = ElementSyncData<QuestionData, Uuid>;
pub type QuestionOptionsSyncData = ElementSyncData<QuestionOptionData, Uuid>;
pub type QuestionTopicsSyncData = ElementSyncData<QuestionTopicData, String>;
pub type QuestionSourcesSyncData = ElementSyncData<QuestionSourceData, String>;
pub type BundlesSyncData = ElementSyncData<BundleData, String>;
pub type IconsSyncData = ElementSyncData<IconData, String>;

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

impl<T: Eq + Hash, K: Eq + Hash> Display for ElementSyncData<T, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(sync: {}; delete: {})",
            self.for_sync.len(),
            self.for_deletion.len()
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncMetadata {
    pub courses: HashMap<String, String>,
    pub questions: HashMap<Uuid, String>,
    pub question_options: HashMap<Uuid, String>,
    pub question_topics: HashSet<String>,
    pub question_sources: HashSet<String>,
    pub bundles: HashMap<String, String>,
    pub icons: HashMap<String, String>,
}
