use schemars::{JsonSchema, Schema};
use serde::{Deserialize, Deserializer, Serialize, de};
use serde_json::json;

fn one_of_condition(schema: &mut Schema) {
    let _ = schema.ensure_object();

    schema.insert(
        "anyOf".to_owned(),
        json!([
            { "required": ["query"] },
            { "required": ["crt_date_from"] },
            { "required": ["crt_date_to"] },
            { "required": ["reg_date_from"] },
            { "required": ["reg_date_to"] },
            { "required": ["lst_date_from"] },
            { "required": ["lst_date_to"] },
        ]),
    );
}

#[derive(Serialize, JsonSchema, Debug, Clone)]
#[schemars(transform = one_of_condition)]
pub struct Condition {
    /// 検索条件。CQL方式で各項目に対する検索クエリーを作成する。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 事例作成日FROM
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crt_date_from: Option<String>,
    /// 事例作成日TO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crt_date_to: Option<String>,
    /// 登録日FROM
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_date_from: Option<String>,
    /// 登録日TO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_date_to: Option<String>,
    /// 最終更新日FROM
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lst_date_from: Option<String>,
    /// 最終更新日TO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lst_date_to: Option<String>,
}

impl<'de> Deserialize<'de> for Condition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            pub query: Option<String>,
            pub crt_date_from: Option<String>,
            pub crt_date_to: Option<String>,
            pub reg_date_from: Option<String>,
            pub reg_date_to: Option<String>,
            pub lst_date_from: Option<String>,
            pub lst_date_to: Option<String>,
        }

        let raw = Raw::deserialize(deserializer)?;

        if raw.query.is_none()
            && raw.crt_date_from.is_none()
            && raw.crt_date_to.is_none()
            && raw.reg_date_from.is_none()
            && raw.reg_date_to.is_none()
            && raw.lst_date_from.is_none()
            && raw.lst_date_to.is_none()
        {
            return Err(de::Error::custom(
                "query, crt_date_from, crt_date_to, reg_date_from, reg_date_to, lst_date_from, lst_date_to のうち少なくとも1つは指定してください",
            ));
        }

        let Raw {
            query,
            crt_date_from,
            crt_date_to,
            reg_date_from,
            reg_date_to,
            lst_date_from,
            lst_date_to,
        } = raw;
        Ok(Self {
            query,
            crt_date_from,
            crt_date_to,
            reg_date_from,
            reg_date_to,
            lst_date_from,
            lst_date_to,
        })
    }
}
