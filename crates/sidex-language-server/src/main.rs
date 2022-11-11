use dashmap::DashMap;
use ropey::Rope;
use sidex_syntax::{
    source::SourceStorage,
    tokens::{self, tokenize, Token},
};
use tower_lsp::{
    jsonrpc,
    lsp_types::{
        DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
        DocumentFilter, InitializeParams, InitializeResult, InitializedParams, MessageType,
        SemanticToken, SemanticTokens, SemanticTokensFullOptions, SemanticTokensLegend,
        SemanticTokensOptions, SemanticTokensParams, SemanticTokensRegistrationOptions,
        SemanticTokensResult, SemanticTokensServerCapabilities, ServerCapabilities, ServerInfo,
        TextDocumentRegistrationOptions, TextDocumentSyncCapability, TextDocumentSyncKind,
    },
    Client, LanguageServer, LspService, Server,
};
use url::Url;

pub mod semantic_tokens;

#[derive(Debug, Default)]
pub struct State {
    source: DashMap<Url, String>,
}

#[derive(Debug)]
pub struct Srv {
    client: Client,
    state: State,
}

#[tower_lsp::async_trait]
impl LanguageServer for Srv {
    async fn initialize(&self, _: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                // 🔮 In the future we may want to support incremental updates.
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
                        SemanticTokensRegistrationOptions {
                            text_document_registration_options: TextDocumentRegistrationOptions {
                                document_selector: Some(vec![DocumentFilter {
                                    language: Some("sidex".to_owned()),
                                    scheme: Some("file".to_owned()),
                                    pattern: None,
                                }]),
                            },
                            semantic_tokens_options: SemanticTokensOptions {
                                work_done_progress_options: Default::default(),
                                legend: SemanticTokensLegend {
                                    token_types: semantic_tokens::LEGEND.into(),
                                    token_modifiers: Default::default(),
                                },
                                range: Some(false),
                                full: Some(SemanticTokensFullOptions::Bool(true)),
                            },
                            static_registration_options: Default::default(),
                        },
                    ),
                ),

                ..ServerCapabilities::default()
            },
            server_info: Some(ServerInfo {
                name: "Sidex Language Server".to_owned(),
                version: Some(env!("CARGO_PKG_VERSION").to_owned()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Language server has been initialized!")
            .await;
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> jsonrpc::Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;

        self.client
            .log_message(
                MessageType::INFO,
                format!("Get semantic tokens for {}.", uri),
            )
            .await;

        let source = self.state.source.get(&uri).unwrap();

        let rope = Rope::from_str(source.value().as_str());

        let mut storage = SourceStorage::new();
        let id = storage.insert(source.value().clone(), None);

        let mut previous_line = 0;
        let mut previous_start = 0;
        let tokens = tokenize(&storage[id])
            .0
            .unwrap()
            .iter()
            .filter_map(|token| {
                let line = rope.try_char_to_line(token.start()).unwrap();
                let line_start = rope.try_line_to_char(line).unwrap();
                let start = token.start() - line_start;
                let token_type = match &token.kind {
                    tokens::TokenKind::Keyword(_) => Some(0),
                    tokens::TokenKind::Literal(literal) => match literal {
                        tokens::Literal::Numeric { .. } => Some(1),
                        tokens::Literal::String(_) => Some(2),
                        tokens::Literal::Boolean(_) => Some(0),
                    },
                    _ => None,
                }?;
                let semantic_token = SemanticToken {
                    delta_line: (line - previous_line).try_into().unwrap(),
                    delta_start: if start >= previous_start {
                        start - previous_start
                    } else {
                        start
                    }
                    .try_into()
                    .unwrap(),
                    length: (token.end() - token.start()).try_into().unwrap(),
                    token_type,
                    token_modifiers_bitset: 0,
                };
                previous_line = line;
                previous_start = start;
                Some(semantic_token)
            })
            .collect();

        let _ = Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: tokens,
        }));
        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(
                MessageType::INFO,
                format!("File `{}` opened.", params.text_document.uri),
            )
            .await;
        self.state
            .source
            .insert(params.text_document.uri, params.text_document.text);
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.client
            .log_message(
                MessageType::INFO,
                format!("File `{}` changed.", params.text_document.uri),
            )
            .await;
        self.state.source.insert(
            params.text_document.uri,
            std::mem::take(&mut params.content_changes[0].text),
        );
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.client
            .log_message(
                MessageType::INFO,
                format!("File `{}` closed.", params.text_document.uri),
            )
            .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Srv {
        client,
        state: Default::default(),
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await
}
