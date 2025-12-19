use crate::crd::{
    Bibl, CrdCollectionResult, CrdError, CrdManualResult, CrdProfileResult, CrdReferenceResult,
    CrdResult, CrdResultSet, CrdSystem, CrdSystemWithoutSysId, NdcClass,
};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema, Debug, Clone)]
pub struct CrdSearchResponse {
    pub hit_count: i32,
    pub cursor_position: i32,
    pub results_returned: i32,
    pub results: Vec<CrdSearchResult>,
}

#[derive(Serialize, JsonSchema, Debug, Clone)]
pub enum CrdSearchResult {
    Reference {
        url: String,
        question: String,
        registration_id: String,
        answer: String,
        created_at: String,
        is_solution: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        keywords: Option<Vec<String>>,
        /// 分類
        #[serde(skip_serializing_if = "Option::is_none")]
        classes: Option<Vec<NdcClass>>,
        /// 調査種別
        ///
        /// 「文献紹介」「事実調査」「書誌的事項調査」「所蔵調査」「所蔵機関調査」「利用案内」「その他」または任意の文字列
        #[serde(skip_serializing_if = "Option::is_none")]
        survey_type: Option<String>,
        /// 内容種別
        ///
        /// 「郷土」「人物」「言葉」「地名」または任意の文字列
        #[serde(skip_serializing_if = "Option::is_none")]
        content_type: Option<String>,
        /// 参考資料
        #[serde(skip_serializing_if = "Option::is_none")]
        bibls: Option<Vec<Bibl>>,
        /// 回答プロセス
        #[serde(skip_serializing_if = "Option::is_none")]
        answer_process: Option<String>,
        /// 照会先
        #[serde(skip_serializing_if = "Option::is_none")]
        referrals: Option<Vec<String>>,
        /// 事前調査事項
        #[serde(skip_serializing_if = "Option::is_none")]
        pre_survey: Option<String>,
        /// 備考
        #[serde(skip_serializing_if = "Option::is_none")]
        note: Option<String>,
        /// 質問者区分
        #[serde(skip_serializing_if = "Option::is_none")]
        questioner_type: Option<String>,
        /// 寄与者
        #[serde(skip_serializing_if = "Option::is_none")]
        contributors: Option<Vec<String>>,
        /// その他の項目(システム管理項目)
        system: CrdSystem,
    },
    Manual {
        url: String,
        theme: String,
        registration_id: String,
        guide: String,
        created_at: String,
        is_completed: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        keywords: Option<Vec<String>>,
        /// 分類
        #[serde(skip_serializing_if = "Option::is_none")]
        classes: Option<Vec<NdcClass>>,
        /// 参考資料
        #[serde(skip_serializing_if = "Option::is_none")]
        bibls: Option<Vec<Bibl>>,
        /// 備考
        #[serde(skip_serializing_if = "Option::is_none")]
        note: Option<String>,
        /// その他の項目(システム管理項目)
        system: CrdSystem,
    },
    Collection {
        url: String,
        name: String,
        name_kana: String,
        registration_id: String,
        content: String,
        /// 来歴
        #[serde(skip_serializing_if = "Option::is_none")]
        origin: Option<String>,
        /// 利用条件
        #[serde(skip_serializing_if = "Option::is_none")]
        restriction: Option<String>,
        /// 目録等
        #[serde(skip_serializing_if = "Option::is_none")]
        catalog: Option<String>,
        /// 紹介文献
        #[serde(skip_serializing_if = "Option::is_none")]
        literature: Option<String>,
        /// 所蔵点数
        number: Option<String>,
        is_continued: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        keywords: Option<Vec<String>>,
        /// 分類
        #[serde(skip_serializing_if = "Option::is_none")]
        classes: Option<Vec<NdcClass>>,
        /// 備考
        #[serde(skip_serializing_if = "Option::is_none")]
        note: Option<String>,
        /// その他の項目(システム管理項目)
        system: CrdSystem,
    },
    Profile {
        url: String,
        /// 図書館名(館種コード)
        // todo: enum?
        library_type: String,
        library_name: String,
        library_name_kana: String,
        library_name_abbr: String,
        zip_code: String,
        address_prefecture: String,
        address_city: String,
        address_street: String,
        tel1: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        tel1_note: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tel2: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tel2_note: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tel3: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tel3_note: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        fax: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        e_mail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        homepage: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        open_info: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        restriction: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        outline: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        feature: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        notes: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        access: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        isil: Option<String>,
        /// その他の項目(システム管理項目)
        system: CrdSystemWithoutSysId,
    },
}

impl From<CrdResultSet> for Result<CrdSearchResponse, ErrorData> {
    fn from(value: CrdResultSet) -> Self {
        if value.results_cd == 0 {
            Ok(CrdSearchResponse {
                hit_count: value.hit_num.unwrap(),
                cursor_position: value.results_get_position,
                results_returned: value.results_num,
                results: value
                    .result
                    .unwrap()
                    .into_iter()
                    .map(|x| x.item.into())
                    .collect(),
            })
        } else {
            let errors = value
                .err_list
                .unwrap_or_default()
                .into_iter()
                .map(|e| {
                    ErrorData::invalid_request(
                        e.err_item.err_msg.clone(),
                        Some(serde_json::to_value(e.err_item).unwrap()),
                    )
                })
                .collect::<Vec<ErrorData>>();
            Err(errors
                .into_iter()
                .next()
                .unwrap_or_else(|| ErrorData::internal_error("Unknown error".to_string(), None)))
        }
    }
}
impl From<CrdResult> for CrdSearchResult {
    fn from(value: CrdResult) -> Self {
        match value {
            CrdResult::Reference(CrdReferenceResult {
                url,
                question,
                reg_id,
                answer,
                crt_date,
                solution,
                keywords,
                classes,
                res_type,
                con_type,
                bibls,
                ans_proc,
                referrals,
                pre_res,
                note,
                ptn_type,
                contri,
                system,
            }) => CrdSearchResult::Reference {
                url,
                question,
                registration_id: reg_id,
                answer,
                created_at: crt_date,
                is_solution: solution == Some(0),
                keywords,
                classes,
                survey_type: res_type,
                content_type: con_type,
                bibls,
                answer_process: ans_proc,
                referrals,
                pre_survey: pre_res,
                note,
                questioner_type: ptn_type,
                contributors: contri,
                system,
            },
            CrdResult::Manual(CrdManualResult {
                url,
                theme,
                reg_id,
                guide,
                crt_date,
                completion,
                keywords,
                classes,
                bibls,
                note,
                system,
            }) => CrdSearchResult::Manual {
                url,
                theme,
                registration_id: reg_id,
                guide,
                created_at: crt_date,
                is_completed: completion == Some(0),
                keywords,
                classes,
                bibls,
                note,
                system,
            },
            CrdResult::Collection(CrdCollectionResult {
                url,
                col_name,
                pro_key,
                reg_id,
                outline,
                origin,
                restriction,
                catalog,
                literature,
                number,
                r#continue,
                keywords,
                classes,
                note,
                system,
            }) => CrdSearchResult::Collection {
                url,
                name: col_name,
                name_kana: pro_key,
                registration_id: reg_id,
                content: outline,
                origin,
                restriction,
                catalog,
                literature,
                number,
                is_continued: r#continue == Some("0".to_string()),
                keywords,
                classes,
                note,
                system,
            },
            CrdResult::Profile(CrdProfileResult {
                url,
                ty,
                name,
                abbr,
                pro_key,
                zip_code,
                add_pref,
                add_city,
                add_street,
                tel1,
                tel1_note,
                tel2,
                tel2_note,
                tel3,
                tel3_note,
                fax,
                e_mail,
                lib_url,
                open_info,
                restriction,
                outline,
                feature,
                notes,
                access,
                isil,
                system,
            }) => CrdSearchResult::Profile {
                url,
                library_type: ty,
                library_name: name,
                library_name_kana: pro_key,
                library_name_abbr: abbr,
                zip_code,
                address_prefecture: add_pref,
                address_city: add_city,
                address_street: add_street,
                tel1,
                tel1_note,
                tel2,
                tel2_note,
                tel3,
                tel3_note,
                fax,
                e_mail,
                homepage: lib_url,
                open_info,
                restriction,
                outline,
                feature,
                notes,
                access,
                isil,
                system,
            },
        }
    }
}
