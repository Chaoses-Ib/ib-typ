import * as ib from 'ib-typ-ide';
import * as vscode from 'vscode';
import { typstDocumentSelector } from './util';
import { PasteResourceProvider } from './paste';

export function activate(context: vscode.ExtensionContext) {
  context.subscriptions.push(
    vscode.languages.registerDocumentPasteEditProvider(typstDocumentSelector, new PasteResourceProvider(), {
      providedPasteEditKinds: [PasteResourceProvider.kind],
      pasteMimeTypes: PasteResourceProvider.mimeTypes,
    })
  );

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
