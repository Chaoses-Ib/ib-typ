/// Insert a line break after every CJK quote (`」`).
/// 
/// = Examples
/// #example(`
/// #show: quote-cjk-line-break
/// 「勝ち取るんだ最高の学園生活を！」
/// 「君に恋をしてしまったんだ…」
/// 「待って！　友達どこいった！？」
/// `)
#let quote-cjk-line-break(it) = {
  show regex(`」 ?`.text): "」\n"
  // TODO: Doesn't work
  show " 「": "「"
  it
}
