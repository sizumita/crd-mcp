mod condition;

pub(crate) use crate::req::condition::Condition;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

fn default_results_num() -> i8 {
    100
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema)]
pub struct CrdSearchRequest {
    /// 検索対象を設定する。
    #[serde(rename = "type")]
    pub ty: ReqType,
    #[serde(flatten)]
    pub condition: Condition,
    /// 提供館コード
    /// 完全一致で検索する。提供館名で検索を行いたい場合、queryにて指定を行う。
    /// 参加館プロファイルの図書館コードも対象とする。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lib_id: Option<String>,
    /// 検索対象の図書館グループを指定する。
    /// 指定がない場合は全ての館から検索を行う。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lib_group: Option<LibGroup>,
    /// 検索結果取得位置
    ///
    /// 検索結果の取得開始位置を0からのインデックスで指定する。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_get_position: Option<i32>,
    /// 検索結果返却件数
    ///
    /// contextの圧迫の制限のため、最大100件まで。デフォルトは100件。
    #[serde(default = "default_results_num")]
    #[schemars(range(min = 0, max = 100))]
    pub results_num: i8,
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

impl Display for ReqType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ReqType::Reference => "reference",
            ReqType::Manual => "manual",
            ReqType::Collection => "collection",
            ReqType::Profile => "profile",
            ReqType::All => "all",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LibGroup {
    /// 全館
    All,
    /// 国立国会図書館
    Ndl,
    /// 公共図書館
    Public,
    /// 大学図書館
    Academic,
    /// 専門図書館
    Special,
    /// 学校図書館
    School,
    /// アーカイブズ
    Archives,
}

impl Display for LibGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LibGroup::All => "all",
            LibGroup::Ndl => "ndl",
            LibGroup::Public => "public",
            LibGroup::Academic => "academic",
            LibGroup::Special => "special",
            LibGroup::School => "school",
            LibGroup::Archives => "archives",
        };
        write!(f, "{}", s)
    }
}
