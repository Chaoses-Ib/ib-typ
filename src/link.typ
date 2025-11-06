#import "util.typ"

/// A shorter, body-first, Markdown-like `link()`.
/// 
/// = Examples
/// == URL
/// #example(`#a[ビジュアルノベル - Wikipedia][https://ja.wikipedia.org/wiki/ビジュアルノベル]`)
/// 
/// which is equivalent to:
/// #example(`#link("https://ja.wikipedia.org/wiki/ビジュアルノベル")[ビジュアルノベル - Wikipedia]`)
/// 
/// == Label
/// #example(`#a([Examples], <examples>) ... <examples>`)
/// 
/// - body (content):
/// - url (content, label):
#let a(body, url) = {
  // to-string() is needed for Unicode
  // [#body #util.to-string(url) #url.fields()]
  if type(url) == label {
    return link(url, body)
  }
  link(util.to-string(url), body)
}
