use crate::req::CrdSearchRequest;
use crate::res::CrdSearchResponse;
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolResult, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
};
use rmcp::{ErrorData, ServerHandler, tool, tool_handler, tool_router};

#[derive(Debug, Clone)]
pub struct CrdService {
    pub http: reqwest::Client,
    tool_router: ToolRouter<Self>,
}

impl CrdService {
    pub fn new() -> CrdService {
        CrdService {
            http: reqwest::Client::new(),
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl CrdService {
    #[tool(
        description = "レファレンス協同データベースシステム(CRD)を検索する。各データを表示する際は、提供館名も明示してください。"
    )]
    pub async fn search(
        &self,
        request: Parameters<CrdSearchRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        let k = self
            .crd_search(request.0)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        let i = Result::<CrdSearchResponse, ErrorData>::from(k)?;
        Ok(CallToolResult::structured(serde_json::to_value(i).unwrap()))
    }
}

#[tool_handler]
impl ServerHandler for CrdService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(env!("CARGO_PKG_DESCRIPTION").to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::req::{Condition, ReqType};
    #[tokio::test]
    async fn test_crd_search() {
        let service = super::CrdService::new();
        let req = super::CrdSearchRequest {
            ty: ReqType::Reference,
            condition: Condition {
                query: Some("question any 北海道".to_string()),
                crt_date_from: None,
                crt_date_to: None,
                reg_date_from: None,
                reg_date_to: None,
                lst_date_from: None,
                lst_date_to: None,
            },
            lib_id: None,
            lib_group: None,
            results_get_position: None,
            results_num: 100,
        };
        let res = service.crd_search(req).await.unwrap();
        assert!(res.hit_num.unwrap() > 0);
    }

    #[tokio::test]
    async fn test_crd_lib_search() {
        let service = super::CrdService::new();
        let req = super::CrdSearchRequest {
            ty: ReqType::Profile,
            condition: Condition {
                query: Some("lib-name any 長野".to_string()),
                crt_date_from: None,
                crt_date_to: None,
                reg_date_from: None,
                reg_date_to: None,
                lst_date_from: None,
                lst_date_to: None,
            },
            lib_id: None,
            lib_group: None,
            results_get_position: None,
            results_num: 100,
        };
        let res = service.crd_search(req).await.unwrap();
        assert!(res.hit_num.unwrap() > 0);
    }

    #[tokio::test]
    async fn test_crd_search_err() {
        let service = super::CrdService::new();
        let req = super::CrdSearchRequest {
            ty: ReqType::Reference,
            condition: Condition {
                query: Some("北海道".to_string()),
                crt_date_from: None,
                crt_date_to: None,
                reg_date_from: None,
                reg_date_to: None,
                lst_date_from: None,
                lst_date_to: None,
            },
            lib_id: None,
            lib_group: None,
            results_get_position: None,
            results_num: 100,
        };
        let res = service.crd_search(req).await.unwrap();
        assert!(res.hit_num.is_none());
    }

    #[tokio::test]
    async fn test_crd_search_no_hit() {
        let service = super::CrdService::new();
        let req = super::CrdSearchRequest {
            ty: ReqType::Reference,
            condition: Condition {
                query: Some(
                    "anywhere = 池袋駅 and anywhere = 雑司が谷 and anywhere = 川".to_string(),
                ),
                crt_date_from: None,
                crt_date_to: None,
                reg_date_from: None,
                reg_date_to: None,
                lst_date_from: None,
                lst_date_to: None,
            },
            lib_id: None,
            lib_group: None,
            results_get_position: None,
            results_num: 10,
        };
        let res = service.crd_search(req).await.unwrap();
        assert_eq!(res.hit_num.unwrap(), 0);
    }
}
