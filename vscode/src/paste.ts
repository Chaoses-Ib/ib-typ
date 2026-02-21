import * as ib from 'ib-typ-ide';
import * as vscode from 'vscode';

export const Mime = {
  textUriList: "text/uri-list",
  textPlain: "text/plain",
} as const;

/**
https://github.com/microsoft/vscode-extension-samples/blob/main/document-paste/src/extension.ts
*/
export class PasteResourceProvider implements vscode.DocumentPasteEditProvider {
  public static readonly kind = vscode.DocumentDropOrPasteEditKind.Text

  public static readonly mimeTypes = [Mime.textPlain];

  provider: ib.PasteEditProvider

  constructor() {
    this.provider = new ib.paste.PasteEditProvider()
  }

  public async provideDocumentPasteEdits(
    document: vscode.TextDocument,
    ranges: readonly vscode.Range[],
    dataTransfer: vscode.DataTransfer,
    context: vscode.DocumentPasteEditContext,
    token: vscode.CancellationToken,
  ): Promise<vscode.DocumentPasteEdit[] | undefined> {
    const textDataTransferItem = dataTransfer.get('text/plain');
    if (!textDataTransferItem) {
      return;
    }

    const text = await textDataTransferItem.asString();
    if (token.isCancellationRequested) {
      return;
    }

    /*
    let insertText: string | vscode.SnippetString = text
    try {
      const typ = ib.link.tree.title_uri_link_list_to_tree_typ(text);
      const snippet = new vscode.SnippetString();
      snippet.appendText(typ);
      insertText = snippet;
    }
    catch (e) {
      // vscode won't log e
      console.debug(e);
      // Otherwise there will be a pasteAs button
      return
    }

    return [
      new vscode.DocumentPasteEdit(insertText, "Link list to tree", PasteResourceProvider.kind),
    ];
    */
    const edits = this.provider.provide_edits(text)
    return edits.map(edit => new vscode.DocumentPasteEdit(
      edit.text,
      edit.title,
      PasteResourceProvider.kind
    ))
  }
}
