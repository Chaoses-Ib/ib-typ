import * as ib from 'ib-typ-ide';
import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
  const disposable = vscode.commands.registerCommand('ib-typ.www.uri_media', () => {
    vscode.window.showInputBox({ prompt: 'Enter URI to process with www.uri_media()' })
    .then(input => {
      if (input !== undefined) {
          const result = ib.www.uri_media(input);
          vscode.window.showInformationMessage(result);
      }
    });
  });
  context.subscriptions.push(disposable);
}

export function deactivate() {
}
