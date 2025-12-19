use crate::req::CrdSearchRequest;
use crate::res::{CrdReferenceResult, CrdResult};
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolResult, Implementation,
    ProtocolVersion, ServerCapabilities, ServerInfo,
};
use rmcp::{ErrorData, ServerHandler, tool, tool_handler, tool_router};

#[derive(Debug, Clone)]
pub struct CrdService {
    tool_router: ToolRouter<Self>,
}

impl CrdService {
    pub fn new() -> CrdService {
        CrdService {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl CrdService {
    #[tool(description = "レファレンス協同データベースシステムを検索する。")]
    pub async fn search(
        _request: Parameters<CrdSearchRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        Ok(CallToolResult::structured(
            serde_json::to_value(vec![CrdResult::Reference {
                url: "".to_string(),
                data: CrdReferenceResult {
                    question: "".to_string(),
                },
            }])
            .unwrap(),
        ))
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
