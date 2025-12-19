mod condition;

use crate::req::condition::Condition;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CrdSearchRequest {
    /// 検索対象を設定する。設定しない場合は全てを対象とする
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ty: Option<ReqType>,
    #[serde(flatten)]
    pub condition: Condition,
}

/// 検索対象
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ReqType {
    /// レファレンス事例
    Reference,
    /// 調べ方マニュアル
    Manual,
    /// 特別コレクション
    Collection,
    /// 参加館プロファイル
    Profile,
    /// すべてを対象
    All,
}
