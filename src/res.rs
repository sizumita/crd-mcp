use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(tag = "type")]
pub enum CrdResult {
    /// レファレンス事例
    Reference {
        data: CrdReferenceResult,
        /// レファレンス事例のURL
        url: String,
    },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct CrdReferenceResult {
    /// 質問
    pub question: String,
}
