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
    /// 検索条件。Contextual Query Languageで各項目に対する検索クエリーを作成する。
    ///
    /// # CQLについて
    ///
    /// CQL（Contextual Query Language）で使われる語句のうち使用できるものは、下記に記載のあるものです:
    ///
    /// ### クエリー対象項目（レファレンス事例, type = "reference"）
    ///
    /// - `anywhere`: 全項目（簡易検索）
    /// - `question`: 質問
    /// - `reg-id`: 管理番号(前方一致)
    /// - `answer`: 回答
    /// - `solution`: 解決／未解決(0: 解決, 1: 未解決)
    /// - `keyword`: キーワード
    /// - `ndc`: NDC分類コード(前方一致)
    /// - `res-type`: 調査種別
    /// - `con-type`: 内容種別
    /// - `bibl-desc`: 参考資料（書誌的事項等）
    /// - `bibl-isbn`: 参考資料（ISBN）
    /// - `ans-proc`: 回答プロセス
    /// - `referral`: 照会先
    /// - `pre-res`: 事前調査結果
    /// - `note`: 備考
    /// - `ptn-type`: 質問者区分
    /// - `contri`: 寄与者
    /// - `sys-id`: 登録番号(完全一致)
    /// - `lib-name`: 提供館名
    ///
    /// ### クエリー対象項目（調べ方マニュアル, type = "manual"）
    ///
    /// - `anywhere`: 全項目（簡易検索）
    /// - `theme`: 調査テーマ
    /// - `reg-id`: 管理番号(前方一致)
    /// - `guide`: 調べ方
    /// - `completion`: 完成／未完成(0: 完成, 1: 未完成)
    /// - `keyword`: キーワード
    /// - `ndc`: NDC分類コード(前方一致)
    /// - `bibl-desc`: 参考資料（書誌的事項等）
    /// - `bibl-isbn`: 参考資料（ISBN）
    /// - `note`: 備考
    /// - `sys-id`: 登録番号(完全一致)
    /// - `lib-name`: 提供館名
    ///
    /// ### クエリー対象項目（特別コレクション, type = "collection"）
    ///
    /// - `anywhere`: 全項目（簡易検索）
    /// - `col-name`: コレクション名、コレクション名ヨミを検索する
    /// - `reg-id`: 管理番号(前方一致)
    /// - `outline`: 内容
    /// - `restriction`: 利用条件
    /// - `catalog`: 目録等
    /// - `literature`: 紹介文献
    /// - `number`: 所蔵点数
    /// - `continue`: 継続／非継続(0: 継続, 1: 非継続)
    /// - `keyword`: キーワード
    /// - `ndc`: NDC分類コード(前方一致)
    /// - `note`: 備考
    /// - `sys-id`: 登録番号(完全一致)
    /// - `lib-name`: 提供館名
    ///
    /// /// ### クエリー対象項目（参加館プロファイル, type = "profile"）
    ///
    /// - `anywhere`: 全項目（簡易検索）
    /// - `lib-type`: 図書館種別
    ///   - 以下の下記コード値、デコード値ともに許可する。
    ///     - 11 国立国会図書館(東京本館)
    ///     - 12 国立国会図書館(関西館)
    ///     - 13 国立国会図書館(国際子ども図書館)
    ///     - 14 国立国会図書館(支部図書館)
    ///     - 21 公共図書館(都道府県立)
    ///     - 22 公共図書館(政令指定都市立)
    ///     - 23 公共図書館(市立・特別区立)
    ///     - 24 公共図書館(町村立)
    ///     - 31 大学図書館(国立大学)
    ///     - 32 大学図書館(公立大学)
    ///     - 33 大学図書館(私立大学)
    ///     - 35 大学図書館(高等専門)
    ///     - 41 専門図書館(国公立)
    ///     - 42 専門図書館(公益法人)
    ///     - 43 専門図書館(企業)
    ///     - 44 専門図書館(その他)
    ///     - 51 学校図書館(高等学校)
    ///     - 52 学校図書館(中学校)
    ///     - 53 学校図書館(小学校)
    ///     - 54 学校図書館(その他)
    ///     - 90 アーカイブズ
    /// - `lib-name`: 図書館名
    ///   - 図書館名（正式）, 図書館名（略式）, 図書館名ヨミを検索する。
    /// - `address`: 住所
    ///   - 住所（都道府県）, 住所（市区町村）, 住所（丁目・番地）（＝住所（検索用））を検索する。
    /// - `open-info`: 開館情報
    /// - `restriction`: 利用条件
    /// - `outline`: 沿革
    /// - `feature`: 特長
    /// - `notes`: 注意事項
    /// - `access`: 交通案内
    /// - `isil`: ISIL
    ///
    /// ## CQL接続子
    ///
    /// - 関係演算子
    ///   - `all`:
    ///     - スペース（全角と半角を区別しない）で区切られた複数のキーワードをAND演算で検索する。例: XXXX all "キーワード1 キーワード2"
    ///   - `any`:
    ///     - ブランクで区切られた複数のキーワードをOR演算で検索する。例: XXXX any "キーワード1 キーワード2"
    ///   - `=`: 等価
    ///     - 指定キーワードでの一致検索。ブランクを含む場合はフレーズでの一致となる。例: XXXX = "キーワード1"
    /// - 論理演算
    ///   - `and`: 論理積
    ///     - 2つの検索句をAND条件で結合する。例: XXXX any "キーワード1" and XXXX = "キーワード2"
    ///   - `or`: 論理和
    ///     - 2つの検索句をOR条件で結合する。例: XXXX any "キーワード1" or XXXX = "キーワード2"
    ///   - `not`: 否定
    ///     - 第1検索句の条件に一致するものから、第2検索句に該当するものを除外。例: XXXX any "キーワード1" not XXXX = "キーワード2"
    ///
    /// ## 検索例
    ///
    /// - 質問（question）に「読書」を含むレファレンス事例を検索
    ///   - `question any 読書`
    /// - 質問（question）に「本」を含み、かつ、回答（answer）に「村上春樹」を含むレファレンス事例を検索
    ///   - `question any 本 and answer any 村上春樹`
    /// - 質問（question）に「伝統芸能」または「音楽」を含み、かつ、解決／未解決（solution）が「解決」（=0またはresolved）を含むレファレンス事例を検索
    ///   - `question any 本 音楽 and solution = 0`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 事例作成日FROM。YYYYMMDDで指定。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crt_date_from: Option<String>,
    /// 事例作成日TO。YYYYMMDDで指定。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crt_date_to: Option<String>,
    /// 登録日FROM。YYYYMMDDで指定。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_date_from: Option<String>,
    /// 登録日TO。YYYYMMDDで指定。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_date_to: Option<String>,
    /// 最終更新日FROM。YYYYMMDDで指定。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lst_date_from: Option<String>,
    /// 最終更新日TO。YYYYMMDDで指定。
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
