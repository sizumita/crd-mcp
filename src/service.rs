use crate::req::CrdSearchRequest;
use crate::res::{CrdReferenceResult, CrdResult};
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
    #[tool(description = "レファレンス協同データベースシステムを検索する。")]
    pub async fn search(
        &self,
        request: Parameters<CrdSearchRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        let k = self
            .crd_search(request.0)
            .await
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::structured(serde_json::to_value(k).unwrap()))
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
        };
        let res = service.crd_search(req).await.unwrap();
        assert!(res.hit_num > 0);
    }
}
