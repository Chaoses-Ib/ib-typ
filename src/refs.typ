#import "layouts.typ": is-multiline
#import "util.typ": to-string-if-raw, to-function

/// Show inline quote attribution.
/// 
/// = Examples
/// #example(`
/// #show: quote-inline-attribution 
/// #quote(attribution: [サグメ])[#lorem(5)]
/// `)
#let quote-inline-attribution(
  it,
  prefix: "@",
) = {
  show quote.where(block: false): it => {
    if it.attribution != none [#prefix#it.attribution: ]
    it
  }
  it
}

/// An enhanced `quote()`:
/// - Shorter to type.
/// - If `body` is a raw block, show its text directly
///   (to avoid explicit line breaks (`\`)).
/// - If `block` is not specified,
///   and the text contains line breaks or the content is multi-line,
///   set `block: true`.
/// - Show inline attribution by default.
/// 
/// = Examples
/// #example(```
/// #q[`
/// 「私は君を喰べに来ました。」
/// 
/// 突如現れた人魚の少女・汐莉は
/// 海辺の街に独り暮らす比名子の手を取り、優しく語りかける。
/// `]
/// ```)
/// 
/// - at (content, label, none):
///   The attribution of this quote, usually the author or source.
///   Can be a label pointing to a bibliography entry or any content.
/// 
///   Empty (`[]`) can be a convention for "anonymous third-party".
/// 
///   See also [`q-at()`] and [`q-i()`].
/// 
/// - show-inline-at (function, none):
#let q(
  body,
  at: none,
  block: none,
  quotes: auto,
  show-inline-at: quote-inline-attribution,
  ..args
) = {
  show: to-function(show-inline-at)
  let body = to-string-if-raw(body)
  let q(block) = quote(
    body,
    attribution: at,
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

/// `q()` with required `at` argument.
/// 
/// Empty (`[]`) can be a convention for "anonymous third-party".
/// 
/// = Examples
/// #example(`#quote(attribution: [サグメ])[#lorem(5)]`)
/// #example(`#q(at: [サグメ])[#lorem(5)]`)
/// 
/// With `q-at()`:
/// #example(`#q-at[サグメ][#lorem(5)]`)
/// #example(`#q-at[][#lorem(5)]`)
/// 
/// Or `q-i()`:
/// #example(`#q-i[#lorem(5)]`)
#let q-at(
  at,
  body,
  block: none,
  quotes: auto,
  ..args
) = q(
  body,
  at: at,
  block: block,
  quotes: quotes,
  ..args
)

/// Basically a shorthand for `q(at: [I])`.
/// 
/// = Examples
/// #example(`#quote(attribution: [I])[#lorem(5)]`)
/// #example(`#q(at: [I])[#lorem(5)]`)
/// #example(`#q-at[I][#lorem(5)]`)
/// #example(`#q-i[#lorem(5)]`)
#let q-i(
  body,
  at: auto,
  block: none,
  quotes: auto,
  ..args
) = {
  if at == auto {
    at = [I]
  }
  q(
    body,
    at: at,
    block: block,
    quotes: quotes,
    show-inline-at: quote-inline-attribution.with(prefix: ""),
    ..args
  )
}
