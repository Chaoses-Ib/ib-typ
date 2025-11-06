#import "util.typ"

/// A shorter, body-first, Markdown-like `link()`.
/// 
/// #example(`#a[ビジュアルノベル - Wikipedia][https://ja.wikipedia.org/wiki/ビジュアルノベル]`)
/// 
/// which is equivalent to:
/// #example(`#link("https://ja.wikipedia.org/wiki/ビジュアルノベル")[ビジュアルノベル - Wikipedia]`)
/// 
/// - body (content):
/// - url (content):
#let a(body, url) = {
  // to-string() is needed for Unicode
  // [#body #util.to-string(url) #url.fields()]
  link(util.to-string(url), body)
}
