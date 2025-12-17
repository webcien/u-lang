// main.rs — U Language Server Protocol Implementation
// MIT License — Copyright (c) 2025 Webcien and U contributors

use lsp_server::{Connection, Message, Request, Response};
use lsp_types::*;
use lsp_types::notification::*;
use std::error::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    env_logger::init();
    log::info!("Starting U Language Server");

    let (connection, io_threads) = Connection::stdio();

    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::FULL,
        )),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(vec![".".to_string()]),
            ..Default::default()
        }),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        ..Default::default()
    })
    .unwrap();

    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    log::info!("Shutting down");
    Ok(())
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    let mut documents: HashMap<Url, String> = HashMap::new();

    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                log::info!("Request: {}", req.method);
            }
            Message::Notification(not) => {
                match not.method.as_str() {
                    "textDocument/didOpen" => {
                        let params: DidOpenTextDocumentParams =
                            serde_json::from_value(not.params).unwrap();
                        documents.insert(
                            params.text_document.uri.clone(),
                            params.text_document.text,
                        );
                    }
                    _ => {}
                }
            }
            Message::Response(_) => {}
        }
    }
    Ok(())
}
