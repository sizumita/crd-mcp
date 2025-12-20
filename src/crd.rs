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
            results_get_position,
            results_num,
        } = request;
        let mut queries = vec![
            ("type", ty.to_string()),
            ("results_num", results_num.to_string()),
        ];
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
        if let Some(results_get_position) = results_get_position {
            queries.push(("results_get_position", results_get_position.to_string()));
        }

        let req = self
            .http
            .get(CRD_API_BASE_URL)
            .query(queries.as_slice())
            .header("User-Agent", format!("crd/{}", env!("CARGO_PKG_VERSION")))
            .build()?;

        let raw_xml = self.http.execute(req).await?.text().await?;
        let result: CrdResultSet = quick_xml::de::from_str(&raw_xml)?;

        Ok(result)
    }
}

#[derive(Deserialize, Debug, Clone)]

pub struct CrdResultSet {
    /// ヒット数
    pub hit_num: Option<i32>,
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
    pub err_list: Option<Vec<ErrorEntry>>,
    /// レファレンス事例リスト
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<ResultEntry>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResultEntry {
    #[serde(rename = "$value")]
    pub item: CrdResult,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ErrorEntry {
    #[serde(rename = "$value")]
    pub err_item: CrdError,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct CrdError {
    /// エラーコード
    ///
    /// エラーコードを格納する。
    pub err_code: String,
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

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum CrdResult {
    Reference(#[serde(rename = "$text")] CrdReferenceResult),
    Manual(#[serde(rename = "$text")] CrdManualResult),
    Collection(#[serde(rename = "$text")] CrdCollectionResult),
    Profile(#[serde(rename = "$text")] CrdProfileResult),
}

#[derive(Deserialize, Debug, Clone)]
pub struct CrdReferenceResult {
    pub url: String,
    pub question: String,
    #[serde(rename = "reg-id")]
    pub reg_id: String,
    pub answer: String,
    #[serde(rename = "crt-date", default)]
    // todo: crt-dateは本来は必須項目であるが、存在しないデータがあるためdefaultを付与
    pub crt_date: String,
    /// 解決/未解決, 0: 解決, 1: 未解決
    pub solution: Option<i8>,
    #[serde(rename = "keyword")]
    pub keywords: Option<Vec<String>>,
    pub classes: Option<Vec<NdcClass>>,
    /// 調査種別
    ///
    /// 「文献紹介」「事実調査」「書誌的事項調査」「所蔵調査」「所蔵機関調査」「利用案内」「その他」または任意の文字列
    #[serde(rename = "res-type", skip_serializing_if = "Option::is_none")]
    pub res_type: Option<String>,
    /// 内容種別
    ///
    /// 「郷土」「人物」「言葉」「地名」または任意の文字列
    #[serde(rename = "con-type", skip_serializing_if = "Option::is_none")]
    pub con_type: Option<String>,
    #[serde(rename = "bibl")]
    pub bibls: Option<Vec<Bibl>>,
    /// 回答プロセス
    #[serde(rename = "ans-proc", skip_serializing_if = "Option::is_none")]
    pub ans_proc: Option<String>,
    /// 照会先
    #[serde(rename = "referral")]
    pub referrals: Option<Vec<String>>,
    /// 事前調査事項
    #[serde(rename = "pre-res", skip_serializing_if = "Option::is_none")]
    pub pre_res: Option<String>,
    /// 備考
    pub note: Option<String>,
    /// 質問者区分
    #[serde(rename = "ptn-type", skip_serializing_if = "Option::is_none")]
    pub ptn_type: Option<String>,
    /// 寄与者
    pub contri: Option<Vec<String>>,
    /// その他の項目(システム管理項目)
    pub system: CrdSystem,
}

#[derive(Deserialize, JsonSchema, Debug, Clone)]
pub struct CrdManualResult {
    pub url: String,
    /// 調査テーマ
    pub theme: String,
    /// 管理番号
    #[serde(rename = "reg-id")]
    pub reg_id: String,
    /// 調べ方
    pub guide: String,
    /// 調べ方作成日
    #[serde(rename = "crt-date")]
    pub crt_date: String,
    /// 完成/未完成, 0: 完成, 1: 未完成
    pub completion: Option<i8>,
    #[serde(rename = "keyword")]
    pub keywords: Option<Vec<String>>,
    pub classes: Option<Vec<NdcClass>>,
    #[serde(rename = "bibl")]
    pub bibls: Option<Vec<Bibl>>,
    /// 備考
    pub note: Option<String>,
    /// その他の項目(システム管理項目)
    pub system: CrdSystem,
}

#[derive(Deserialize, JsonSchema, Debug, Clone)]
pub struct CrdCollectionResult {
    pub url: String,
    /// コレクション名
    #[serde(rename = "col-name")]
    pub col_name: String,
    /// コレクション名ヨミ
    #[serde(rename = "pro_key")]
    pub pro_key: String,
    /// 管理番号
    #[serde(rename = "reg-id")]
    pub reg_id: String,
    /// 内容
    pub outline: String,
    /// 来歴
    pub origin: Option<String>,
    /// 利用条件
    pub restriction: Option<String>,
    /// 目録等
    pub catalog: Option<String>,
    /// 紹介文献
    pub literature: Option<String>,
    /// 所蔵点数
    pub number: Option<String>,
    /// 継続
    ///
    /// 0（継続有）, 1（継続無）
    pub r#continue: Option<String>,
    #[serde(rename = "keyword")]
    pub keywords: Option<Vec<String>>,
    pub classes: Option<Vec<NdcClass>>,
    /// 備考
    pub note: Option<String>,
    /// その他の項目(システム管理項目)
    pub system: CrdSystem,
}

#[derive(Deserialize, JsonSchema, Debug, Clone)]
pub struct CrdProfileResult {
    pub url: String,
    /// 館種コード
    #[serde(rename = "lib-type")]
    pub ty: String,
    /// 図書館名（正式）
    #[serde(rename = "lib-name")]
    pub name: String,
    /// 図書館名（略式）
    pub abbr: String,
    /// 図書館名ヨミ
    #[serde(rename = "pro-key")]
    pub pro_key: String,
    /// 郵便番号
    #[serde(rename = "zip-code")]
    pub zip_code: String,
    /// 住所（都道府県）
    #[serde(rename = "add-pref")]
    pub add_pref: String,
    /// 住所（市区町村）
    #[serde(rename = "add-city")]
    pub add_city: String,
    /// 住所（丁目・番地）
    #[serde(rename = "add-street")]
    pub add_street: String,
    /// 電話番号１
    pub tel1: String,
    /// 電話番号１（追加情報）
    #[serde(rename = "tel1-note")]
    pub tel1_note: Option<String>,
    /// 電話番号２
    pub tel2: Option<String>,
    /// 電話番号２（追加情報）
    #[serde(rename = "tel2-note")]
    pub tel2_note: Option<String>,
    /// 電話番号３
    pub tel3: Option<String>,
    /// 電話番号３（追加情報）
    #[serde(rename = "tel3-note")]
    pub tel3_note: Option<String>,
    /// FAX番号
    pub fax: Option<String>,
    /// e-mailアドレス
    #[serde(rename = "e-mail")]
    pub e_mail: Option<String>,
    /// 図書館URL
    #[serde(rename = "lib-url")]
    pub lib_url: Option<String>,
    /// 開館情報
    #[serde(rename = "open-info")]
    pub open_info: Option<String>,
    /// 利用条件
    pub restriction: Option<String>,
    /// 沿革
    pub outline: Option<String>,
    /// 特色
    pub feature: Option<String>,
    /// 注意事項
    pub notes: Option<String>,
    /// 交通案内
    pub access: Option<String>,
    /// ISIL
    pub isil: Option<String>,
    /// その他の項目(システム管理項目)
    pub system: CrdSystemWithoutSysId,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct CrdSystemWithoutSysId {
    /// 登録日時
    #[serde(rename = "reg-date")]
    pub reg_date: String,
    /// 最終更新日時
    #[serde(rename = "lst-date")]
    pub lst_date: String,
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
