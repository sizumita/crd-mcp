use crate::req::CrdSearchRequest;
use crate::service::CrdService;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const CRD_API_BASE_URL: &str = "https://crd.ndl.go.jp/api/refsearch";

impl CrdService {
    pub async fn crd_search(&self, request: CrdSearchRequest) -> anyhow::Result<CrdResultSet> {
        let CrdSearchRequest {
            ty,
            condition,
            lib_id,
            lib_group,
        } = request;
        let mut queries = vec![("type", ty.to_string())];
        if let Some(query) = &condition.query {
            queries.push(("query", query.to_string()));
        }
        if let Some(query) = &condition.crt_date_from {
            queries.push(("crt-date_from", query.to_string()));
        }
        if let Some(query) = &condition.crt_date_to {
            queries.push(("crt-date_to", query.to_string()));
        }
        if let Some(query) = &condition.reg_date_from {
            queries.push(("reg-date_from", query.to_string()));
        }
        if let Some(query) = &condition.reg_date_to {
            queries.push(("reg-date_to", query.to_string()));
        }
        if let Some(query) = &condition.lst_date_from {
            queries.push(("lst-date_from", query.to_string()));
        }
        if let Some(query) = &condition.lst_date_from {
            queries.push(("lst-date_to", query.to_string()));
        }
        if let Some(lib_id) = lib_id {
            queries.push(("lib_id", lib_id.clone()));
        }
        if let Some(lib_group) = lib_group {
            queries.push(("lib_group", lib_group.to_string()));
        }

        let req = self
            .http
            .get(CRD_API_BASE_URL)
            .query(queries.as_slice())
            .header("User-Agent", format!("crd/{}", env!("CARGO_PKG_VERSION")))
            .build()?;
        eprintln!("{}", req.url());

        let raw_xml = self.http.execute(req).await?.text().await?;
        let result: CrdResultSet = quick_xml::de::from_str(&*raw_xml)?;

        Ok(result)
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct CrdResultSet {
    /// ヒット数
    pub hit_num: i32,
    /// 検索開始位置
    pub results_get_position: i32,
    /// 検索結果返却件数
    pub results_num: i32,
    /// 処理結果コード
    ///
    /// 処理の成功/失敗を格納
    /// 成功：0
    /// エラー：1
    /// ※エラーの詳細はerr_itemに格納
    pub results_cd: i32,
    /// エラー情報リスト
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_list: Option<Vec<CrdError>>,
    /// レファレンス事例リスト
    pub result: Vec<ResultEntry>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct ResultEntry {
    // <result> の子要素を順に全部吸う（混在対応の要）
    #[serde(rename = "$value", default)]
    items: Vec<CrdResult>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct CrdError {
    /// エラーコード
    ///
    /// エラーコードを格納する。
    pub err_code: i32,
    /// エラーフィールド
    ///
    /// エラーが発生したフィールド（検索リクエストのパラメタ）を表示する。
    /// ・queryパラメタ内でのエラーの場合、該当の検索キーが出力される。
    pub err_fld: String,
    /// エラーメッセージ
    ///
    /// エラーメッセージを格納する。
    pub err_msg: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CrdResult {
    Reference(#[serde(rename = "$text")] CrdReferenceResult),
    Manual { url: String },
    Collection { url: String },
    Profile { url: String },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct CrdReferenceResult {
    url: String,
    question: String,
    #[serde(rename = "reg-id")]
    reg_id: String,
    answer: String,
    #[serde(rename = "crt-date", default)]
    // todo: crt-dateは本来は必須項目であるが、存在しないデータがあるためdefaultを付与
    crt_date: String,
    /// 解決/未解決, 0: 解決, 1: 未解決
    solution: Option<i8>,
    #[serde(rename = "keyword", default)]
    keywords: Vec<String>,
    #[serde(default)]
    classes: Vec<NdcClass>,
    /// 調査種別
    ///
    /// 「文献紹介」「事実調査」「書誌的事項調査」「所蔵調査」「所蔵機関調査」「利用案内」「その他」または任意の文字列
    #[serde(rename = "res-type", skip_serializing_if = "Option::is_none")]
    res_type: Option<String>,
    /// 内容種別
    ///
    /// 「郷土」「人物」「言葉」「地名」または任意の文字列
    #[serde(rename = "con-type", skip_serializing_if = "Option::is_none")]
    con_type: Option<String>,
    #[serde(rename = "bibl", default)]
    bibls: Vec<Bibl>,
    /// 回答プロセス
    #[serde(rename = "ans-proc", skip_serializing_if = "Option::is_none")]
    ans_proc: Option<String>,
    /// 照会先
    #[serde(rename = "referral", default)]
    referrals: Vec<String>,
    #[serde(rename = "pre-res", skip_serializing_if = "Option::is_none")]
    pre_res: Option<String>,
    /// 備考
    note: Option<String>,
    /// 質問者区分
    #[serde(rename = "ptn-type", skip_serializing_if = "Option::is_none")]
    ptn_type: Option<String>,
    /// 寄与者
    #[serde(default)]
    contri: Vec<String>,
    /// その他の項目(システム管理項目)
    system: CrdSystem,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct CrdSystem {
    /// 登録日時
    #[serde(rename = "reg-date")]
    pub reg_date: String,
    /// 最終更新日時
    #[serde(rename = "lst-date")]
    pub lst_date: String,
    /// 登録番号
    #[serde(rename = "sys-id")]
    pub sys_id: String,
    /// 提供館コード
    #[serde(rename = "lib-id")]
    pub lib_id: String,
    /// 提供館名
    #[serde(rename = "lib-name")]
    pub lib_name: String,
    /// 関連ファイル数
    ///
    /// 関連ファイルの登録がある場合は登録数を表示する。
    /// 登録がない場合は「0」を返却する。
    #[serde(rename = "file-num")]
    pub file_num: i32,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct NdcClass {
    /// 分類の種類(「NDC」のみ)
    #[serde(rename = "@type")]
    ty: String,
    /// typeで指定された分類のバージョン(例 9(9版を示す))
    #[serde(rename = "@code", skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    /// 分類の番号
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct Bibl {
    /// 書誌的事項(参考資料)
    #[serde(rename = "bibl-desc", skip_serializing_if = "Option::is_none")]
    desc: Option<String>,
    /// ISBN（参考資料）
    #[serde(rename = "bibl-isbn", skip_serializing_if = "Option::is_none")]
    isbn: Option<String>,
    /// 備考(参考資料)
    #[serde(rename = "bibl-note", skip_serializing_if = "Option::is_none")]
    note: Option<String>,
}
