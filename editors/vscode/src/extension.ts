import * as vscode from "vscode"

import {
  Executable,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node"

let client: LanguageClient | undefined = undefined

export function activate(context: vscode.ExtensionContext) {
  const command = process.env.SERVER_PATH || "sidex-language-server"

  const run: Executable = {
    command,
    options: {
      env: {
        ...process.env,
        // eslint-disable-next-line @typescript-eslint/naming-convention
        RUST_LOG: "debug",
      },
    },
  }

  const serverOptions: ServerOptions = { run, debug: run }
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "sidex" }],
  }

  client = new LanguageClient(
    "sidex-language-server",
    "Sidex Language Server",
    serverOptions,
    clientOptions,
  )
  client.start()

  let disposable = vscode.commands.registerCommand(
    "sidex.restartServer",
    () => {
      vscode.window.showInformationMessage("Restarting Sidex language server.")
      client?.restart()
    },
  )

  context.subscriptions.push(disposable)
}

export function deactivate() {
  client?.stop()
}
