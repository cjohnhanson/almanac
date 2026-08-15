//! Serving a library over MCP.
//!
//! The same library that the command line reads is what a client
//! reads. This module carries no behavior of its own: it maps requests
//! onto [`crate::workspace::Workspace`] and shapes the answers.
//!
//! A server offers one or more surfaces, because a client cannot be
//! asked which it understands:
//!
//! - **skills** — the skills extension (`skills/list`, `skills/get`).
//!   The richest surface, and the newest, so the fewest clients
//!   implement it. Almanac fits it closely: it already pins content by
//!   SHA-256, which is what the extension asks a host to verify.
//! - **resources** — one readable resource for each file of each skill.
//! - **tools** — `almanac_list_skills`, `almanac_get_skill`,
//!   `almanac_check`. Every client can call a tool, so this is the
//!   floor that a served library can rely on.

use std::path::Path;
use std::sync::Arc;

use mdstore::mcp::{Access, ServeConfig, Surface, digest_of};
use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, CustomResult,
    ErrorData as McpError, Implementation, InitializeResult, ListResourcesResult, ListToolsResult,
    PaginatedRequestParams, ProtocolVersion, ReadResourceRequestParams, ReadResourceResponse,
    ReadResourceResult, Resource, ResourceContents, ResourcesCapability, ServerCapabilities, Tool,
    ToolsCapability,
};
use rmcp::service::RequestContext;
use rmcp::{RoleServer, ServerHandler};
use serde_json::{Map, Value, json};

use crate::error::Error;
use crate::workspace::Workspace;

/// The URI scheme for a skill and its files, as the extension writes it.
const SCHEME: &str = "skill";

/// One skill, in the shape the skills extension returns.
struct SkillEntry {
    uri: String,
    /// The frontmatter as written, never re-serialized: a client
    /// compares a listing's copy against a fetch's.
    frontmatter: String,
    /// Every file of the skill, each with its own digest. The
    /// extension asks a host to verify content against these, so the
    /// digest is per file, not per skill.
    resources: Vec<(String, String)>,
}

/// A served library.
#[derive(Clone)]
pub struct AlmanacServer {
    config: Arc<ServeConfig>,
}

impl AlmanacServer {
    #[must_use]
    pub fn new(config: ServeConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    fn root(&self) -> &Path {
        &self.config.root
    }

    fn workspace(&self) -> Result<Workspace, Error> {
        Workspace::open(self.root())
    }

    /// Every skill the library reaches, after precedence, in the shape
    /// the extension returns.
    fn skill_entries(&self) -> Result<Vec<SkillEntry>, Error> {
        let ws = self.workspace()?;
        let mut entries = Vec::new();
        for view in ws.skills().iter().filter(|v| !v.shadowed) {
            let text = ws.read_skill(view)?;
            let frontmatter = mdstore::document::split(&text)
                .map(|(fm, _)| fm.to_string())
                .unwrap_or_default();
            let base = format!("{SCHEME}://{}", view.skill.name);
            let mut resources = vec![(
                format!("{base}/SKILL.md"),
                digest_of(text.as_bytes()),
            )];
            for (rel, bytes) in ws.skill_files(view) {
                resources.push((format!("{base}/{rel}"), digest_of(&bytes)));
            }
            entries.push(SkillEntry {
                uri: format!("{base}/SKILL.md"),
                frontmatter,
                resources,
            });
        }
        Ok(entries)
    }

    fn entry_json(entry: &SkillEntry) -> Value {
        json!({
            "uri": entry.uri,
            "frontmatter": entry.frontmatter,
            "resources": entry.resources.iter().map(|(uri, digest)| json!({
                "uri": uri,
                "digest": digest,
            })).collect::<Vec<_>>(),
        })
    }

    /// The text of one file named by a `skill://` URI.
    fn read_uri(&self, uri: &str) -> Result<String, Error> {
        let rest = uri
            .strip_prefix(&format!("{SCHEME}://"))
            .ok_or_else(|| Error::General(format!("'{uri}' is not a skill URI")))?;
        let (name, sub) = rest
            .split_once('/')
            .ok_or_else(|| Error::General(format!("'{uri}' names no file")))?;
        let ws = self.workspace()?;
        let view = ws
            .resolve(name)
            .ok_or_else(|| Error::General(format!("no skill named '{name}'")))?;
        if sub == "SKILL.md" {
            return ws.read_skill(&view);
        }
        // A file beside the SKILL.md, resolved against the library
        // that holds the skill. The name is matched against that
        // library's own files, so a URI cannot reach outside it.
        for (rel, bytes) in ws.skill_files(&view) {
            if rel == sub {
                return String::from_utf8(bytes)
                    .map_err(|e| Error::General(format!("{uri} is not text: {e}")));
            }
        }
        Err(Error::General(format!("'{uri}' names no file")))
    }

    // -- the tool surface --

    fn tool_definitions() -> Vec<Tool> {
        vec![
            Tool::new(
                "almanac_list_skills",
                "List the skills this library reaches, after precedence, with their \
                 descriptions. A client that reads resources can read skill:// URIs instead.",
                Arc::new(empty_schema()),
            ),
            Tool::new(
                "almanac_get_skill",
                "Return the full text of one skill, by name. Optionally a file beside it, \
                 such as references/deep-dive.md.",
                Arc::new(schema_with(&[
                    ("name", "The skill name.", true),
                    ("file", "A file beside the SKILL.md.", false),
                ])),
            ),
            Tool::new(
                "almanac_check",
                "Report the problems in this library: a requirement that names no skill, a \
                 name that two libraries hold, an unreachable library.",
                Arc::new(empty_schema()),
            ),
        ]
    }

    fn call(&self, name: &str, args: &Map<String, Value>) -> Result<String, Error> {
        match name {
            "almanac_list_skills" => {
                let ws = self.workspace()?;
                let listed: Vec<Value> = ws
                    .skills()
                    .iter()
                    .filter(|v| !v.shadowed)
                    .map(|v| {
                        json!({
                            "name": v.skill.name,
                            "description": v.skill.description,
                            "library": if v.store.is_empty() { "(this library)" } else { v.store.as_str() },
                            "requires": v.skill.requires,
                        })
                    })
                    .collect();
                Ok(serde_json::to_string_pretty(&listed).unwrap_or_default())
            }
            "almanac_get_skill" => {
                let skill = args
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| Error::General("name is required".into()))?;
                let uri = args.get("file").and_then(Value::as_str).map_or_else(
                    || format!("{SCHEME}://{skill}/SKILL.md"),
                    |file| format!("{SCHEME}://{skill}/{file}"),
                );
                self.read_uri(&uri)
            }
            "almanac_check" => {
                let ws = self.workspace()?;
                let findings = ws.check(self.root());
                if findings.is_empty() {
                    return Ok("no problems found".into());
                }
                Ok(serde_json::to_string_pretty(&findings).unwrap_or_default())
            }
            other => Err(Error::General(format!("no tool named '{other}'"))),
        }
    }
}

fn empty_schema() -> Map<String, Value> {
    let mut schema = Map::new();
    schema.insert("type".into(), json!("object"));
    schema.insert("properties".into(), json!({}));
    schema
}

fn schema_with(fields: &[(&str, &str, bool)]) -> Map<String, Value> {
    let mut properties = Map::new();
    let mut required = Vec::new();
    for (name, description, is_required) in fields {
        properties.insert(
            (*name).to_string(),
            json!({ "type": "string", "description": description }),
        );
        if *is_required {
            required.push(json!(name));
        }
    }
    let mut schema = Map::new();
    schema.insert("type".into(), json!("object"));
    schema.insert("properties".into(), Value::Object(properties));
    if !required.is_empty() {
        schema.insert("required".into(), Value::Array(required));
    }
    schema
}

fn to_mcp_error(e: &Error) -> McpError {
    McpError::internal_error(e.to_string(), None)
}

impl ServerHandler for AlmanacServer {
    fn get_info(&self) -> InitializeResult {
        let mut capabilities = ServerCapabilities::default();
        if self.config.surfaces.has(Surface::Tools) {
            capabilities.tools = Some(ToolsCapability::default());
        }
        if self.config.surfaces.has(Surface::Resources) {
            capabilities.resources = Some(ResourcesCapability::default());
        }
        if self.config.surfaces.has(Surface::Skills) {
            // The skills extension declares itself here, by identifier.
            // directoryRead stays off: a client walks the resource list
            // that each skill already carries.
            let mut extensions = std::collections::BTreeMap::new();
            extensions.insert("io.modelcontextprotocol/skills".to_string(), Map::new());
            capabilities.extensions = Some(extensions);
        }

        let mut instructions = format!(
            "A curated library of agent skills. Surfaces: {}.",
            self.config.surfaces
        );
        if self.config.access == Access::ReadOnly {
            instructions.push_str(" This library is read-only.");
        }
        if self.config.surfaces.has(Surface::Resources) && self.config.surfaces.has(Surface::Tools)
        {
            instructions.push_str(
                " Prefer reading skill:// resources when you support them; the tools return \
                 the same content.",
            );
        }

        InitializeResult::new(capabilities)
            .with_protocol_version(ProtocolVersion::default())
            .with_server_info({
                let mut info = Implementation::from_build_env();
                info.name.clone_from(&self.config.server_name);
                info.version = env!("CARGO_PKG_VERSION").to_string();
                info
            })
            .with_instructions(instructions)
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        if !self.config.surfaces.has(Surface::Tools) {
            return Ok(ListToolsResult::default());
        }
        Ok(ListToolsResult::with_all_items(Self::tool_definitions()))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, McpError> {
        if !self.config.surfaces.has(Surface::Tools) {
            return Err(McpError::invalid_request("tools are not served here", None));
        }
        let args = request.arguments.unwrap_or_default();
        match self.call(&request.name, &args) {
            Ok(text) => Ok(CallToolResult::success(vec![ContentBlock::text(text)]).into()),
            // A failure the caller can act on comes back as tool
            // content, not a protocol error, so the model can read it.
            Err(e) => {
                Ok(CallToolResult::error(vec![ContentBlock::text(e.to_string())]).into())
            }
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        if !self.config.surfaces.has(Surface::Resources) {
            return Ok(ListResourcesResult::default());
        }
        let entries = self.skill_entries().map_err(|e| to_mcp_error(&e))?;
        let mut resources = Vec::new();
        for entry in &entries {
            for (uri, _) in &entry.resources {
                let mut resource = Resource::new(
                    uri.clone(),
                    uri.rsplit('/').next().unwrap_or(uri).to_string(),
                );
                resource.mime_type = Some("text/markdown".into());
                resources.push(resource);
            }
        }
        Ok(ListResourcesResult::with_all_items(resources))
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResponse, McpError> {
        if !self.config.surfaces.has(Surface::Resources) {
            return Err(McpError::invalid_request(
                "resources are not served here",
                None,
            ));
        }
        let text = self.read_uri(&request.uri).map_err(|e| to_mcp_error(&e))?;
        Ok(ReadResourceResult::new(vec![ResourceContents::text(text, request.uri)]).into())
    }

    /// The skills extension. Its methods are not part of the base
    /// protocol, so they arrive here.
    async fn on_custom_request(
        &self,
        request: rmcp::model::CustomRequest,
        _context: RequestContext<RoleServer>,
    ) -> Result<CustomResult, McpError> {
        if !self.config.surfaces.has(Surface::Skills) {
            return Err(McpError::invalid_request(
                format!("{} is not served here", request.method),
                None,
            ));
        }
        match request.method.as_str() {
            "skills/list" => {
                let entries = self.skill_entries().map_err(|e| to_mcp_error(&e))?;
                Ok(CustomResult::new(json!({
                    "skills": entries.iter().map(Self::entry_json).collect::<Vec<_>>(),
                })))
            }
            "skills/get" => {
                let uri = request
                    .params
                    .as_ref()
                    .and_then(|p| p.get("uri"))
                    .and_then(Value::as_str)
                    .ok_or_else(|| McpError::invalid_params("uri is required", None))?;
                let entries = self.skill_entries().map_err(|e| to_mcp_error(&e))?;
                let entry = entries.iter().find(|e| e.uri == uri).ok_or_else(|| {
                    McpError::invalid_params(format!("no skill at {uri}"), None)
                })?;
                Ok(CustomResult::new(json!({ "skill": Self::entry_json(entry) })))
            }
            other => Err(McpError::invalid_request(
                format!("unknown method {other}"),
                None,
            )),
        }
    }
}

/// Serve a library, on stdio or over HTTP.
///
/// With no bind address the server speaks on stdin and stdout, for a
/// client that starts it as a child process. With one, it listens for
/// clients that connect to it, which is what serving a library to
/// somebody else needs.
pub fn run(config: ServeConfig, bind: Option<&str>) -> Result<(), Error> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| Error::General(format!("cannot start the runtime: {e}")))?;
    runtime.block_on(async move {
        match bind {
            None => serve_stdio(config).await,
            Some(addr) => serve_http(config, addr).await,
        }
    })
}

async fn serve_stdio(config: ServeConfig) -> Result<(), Error> {
    use rmcp::ServiceExt;
    let server = AlmanacServer::new(config);
    let service = server
        .serve(rmcp::transport::stdio())
        .await
        .map_err(|e| Error::General(format!("cannot serve on stdio: {e}")))?;
    service
        .waiting()
        .await
        .map_err(|e| Error::General(format!("the connection failed: {e}")))?;
    Ok(())
}

async fn serve_http(config: ServeConfig, addr: &str) -> Result<(), Error> {
    use rmcp::transport::streamable_http_server::{
        StreamableHttpService, session::local::LocalSessionManager,
    };

    let read_only = config.access == Access::ReadOnly;
    let server = AlmanacServer::new(config);
    let service = StreamableHttpService::new(
        move || Ok(server.clone()),
        LocalSessionManager::default().into(),
        rmcp::transport::streamable_http_server::StreamableHttpServerConfig::default(),
    );

    let router = axum::Router::new().nest_service("/mcp", service);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| Error::General(format!("cannot listen on {addr}: {e}")))?;
    let bound = listener
        .local_addr()
        .map_err(|e| Error::General(format!("cannot read the address: {e}")))?;
    // The operator needs to know what was actually opened, and whether
    // it accepts writes, before a client connects.
    eprintln!(
        "almanac serving on http://{bound}/mcp ({})",
        if read_only { "read-only" } else { "read-write" }
    );
    axum::serve(listener, router)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .map_err(|e| Error::General(format!("the server stopped: {e}")))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mdstore::mcp::Surfaces;
    use std::path::PathBuf;

    fn fixture() -> PathBuf {
        let base = std::env::temp_dir().join(format!(
            "almanac-serve-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&base);
        let skill = base.join("skills").join("writing");
        std::fs::create_dir_all(skill.join("references")).unwrap();
        std::fs::write(
            skill.join("SKILL.md"),
            "---\nname: writing\ndescription: How to write.\n---\n\n# Writing\n",
        )
        .unwrap();
        std::fs::write(skill.join("references/deep-dive.md"), "# Deep dive\n").unwrap();
        std::fs::write(base.join("almanac.yml"), "library: skills\nskills: []\n").unwrap();
        base
    }

    fn server(surfaces: Surfaces) -> AlmanacServer {
        AlmanacServer::new(ServeConfig::read_only(fixture(), surfaces, "almanac"))
    }

    #[test]
    fn a_skill_entry_carries_verbatim_frontmatter() {
        let s = server(Surfaces::skills_and_tools());
        let entries = s.skill_entries().unwrap();
        assert_eq!(entries.len(), 1);
        // Verbatim: not re-serialized, so a listing and a fetch agree.
        assert_eq!(
            entries[0].frontmatter,
            "name: writing\ndescription: How to write."
        );
        assert_eq!(entries[0].uri, "skill://writing/SKILL.md");
    }

    #[test]
    fn every_file_of_a_skill_gets_its_own_digest() {
        let s = server(Surfaces::skills_and_tools());
        let entries = s.skill_entries().unwrap();
        let uris: Vec<&str> = entries[0]
            .resources
            .iter()
            .map(|(u, _)| u.as_str())
            .collect();
        assert_eq!(
            uris,
            vec![
                "skill://writing/SKILL.md",
                "skill://writing/references/deep-dive.md"
            ]
        );
        for (_, digest) in &entries[0].resources {
            assert!(digest.starts_with("sha256:"), "{digest}");
        }
        // Two different files, two different digests.
        assert_ne!(entries[0].resources[0].1, entries[0].resources[1].1);
    }

    #[test]
    fn a_uri_reads_the_file_it_names() {
        let s = server(Surfaces::skills_and_tools());
        assert!(
            s.read_uri("skill://writing/SKILL.md")
                .unwrap()
                .contains("# Writing")
        );
        assert!(
            s.read_uri("skill://writing/references/deep-dive.md")
                .unwrap()
                .contains("# Deep dive")
        );
    }

    #[test]
    fn a_uri_that_names_nothing_is_refused() {
        let s = server(Surfaces::skills_and_tools());
        for bad in [
            "skill://writing/nope.md",
            "skill://nosuch/SKILL.md",
            "http://writing/SKILL.md",
            "skill://writing",
        ] {
            assert!(s.read_uri(bad).is_err(), "{bad} must be refused");
        }
    }

    #[test]
    fn a_uri_cannot_escape_the_skill_directory() {
        let s = server(Surfaces::skills_and_tools());
        for bad in [
            "skill://writing/../../almanac.yml",
            "skill://writing/../writing/SKILL.md",
        ] {
            assert!(s.read_uri(bad).is_err(), "{bad} must be refused");
        }
    }

    #[test]
    fn the_tool_surface_answers_the_same_content() {
        let s = server(Surfaces::tools_only());
        let listed = s.call("almanac_list_skills", &Map::new()).unwrap();
        assert!(listed.contains("writing"), "{listed}");

        let mut args = Map::new();
        args.insert("name".into(), json!("writing"));
        let text = s.call("almanac_get_skill", &args).unwrap();
        assert!(text.contains("# Writing"), "{text}");

        args.insert("file".into(), json!("references/deep-dive.md"));
        let sub = s.call("almanac_get_skill", &args).unwrap();
        assert!(sub.contains("# Deep dive"), "{sub}");
    }

    #[test]
    fn a_tool_call_without_its_argument_fails() {
        let s = server(Surfaces::tools_only());
        assert!(s.call("almanac_get_skill", &Map::new()).is_err());
        assert!(s.call("almanac_nonexistent", &Map::new()).is_err());
    }

    #[test]
    fn check_answers_through_the_tool_surface() {
        let s = server(Surfaces::tools_only());
        let out = s.call("almanac_check", &Map::new()).unwrap();
        assert_eq!(out, "no problems found");
    }

    #[test]
    fn the_declared_capabilities_follow_the_surfaces() {
        let tools_only = server(Surfaces::tools_only()).get_info();
        assert!(tools_only.capabilities.tools.is_some());
        assert!(tools_only.capabilities.resources.is_none());

        let both = server(Surfaces::resources_and_tools()).get_info();
        assert!(both.capabilities.tools.is_some());
        assert!(both.capabilities.resources.is_some());
    }

    #[test]
    fn a_read_only_server_says_so() {
        let info = server(Surfaces::tools_only()).get_info();
        let instructions = info.instructions.unwrap_or_default();
        assert!(instructions.contains("read-only"), "{instructions}");
    }

    #[test]
    fn serving_both_surfaces_points_at_the_cheaper_one() {
        let info = server(Surfaces::resources_and_tools()).get_info();
        let instructions = info.instructions.unwrap_or_default();
        assert!(instructions.contains("Prefer reading"), "{instructions}");
    }
}
