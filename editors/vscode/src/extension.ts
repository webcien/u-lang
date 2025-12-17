// extension.ts — U Language VS Code Extension
// MIT License — Copyright (c) 2025 Webcien and U contributors

import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
  console.log('U Language extension is now active');

  // Get the LSP server path from configuration
  const config = workspace.getConfiguration('u');
  const serverPath = config.get<string>('lsp.path') || 'u-lsp';

  // Server options
  const serverOptions: ServerOptions = {
    run: { command: serverPath, transport: TransportKind.stdio },
    debug: { command: serverPath, transport: TransportKind.stdio }
  };

  // Client options
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'u' }],
    synchronize: {
      fileEvents: workspace.createFileSystemWatcher('**/*.ul')
    }
  };

  // Create the language client
  client = new LanguageClient(
    'uLanguageServer',
    'U Language Server',
    serverOptions,
    clientOptions
  );

  // Start the client
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
