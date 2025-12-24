#import "layouts.typ": is-multiline
#import "util.typ": to-string-if-raw

/// An enhanced `quote()`:
/// - Shorter to type.
/// - If `body` is a raw block, show its text directly
///   (to avoid explicit line breaks (`\`)).
/// - If `block` is not specified,
///   and the text contains line breaks or the content is multi-line,
///   set `block: true`.
/// 
/// #example(```
/// #q[`
/// 「私は君を喰べに来ました。」
/// 
/// 突如現れた人魚の少女・汐莉は
/// 海辺の街に独り暮らす比名子の手を取り、優しく語りかける。
/// `]
/// ```)
#let q(
  body,
  attribution: none,
  block: none,
  quotes: auto,
  ..args
) = {
  let body = to-string-if-raw(body)
  let q(block) = quote(
    body,
    attribution: attribution,
    block: block,
    quotes: quotes,
    ..args
  )
  if block != none {
    q(block)
  } else if type(body) == str {
    block = body.contains("\n")
    q(block)
  } else {
    context {
      let block = is-multiline(body)
      q(block)
    }
  }
}
