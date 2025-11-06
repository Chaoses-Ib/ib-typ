#import "uri.typ": uri-host
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

/// Emphasize the authority/host instead of the path/page.
/// 
/// #example(`#a-badge[https://github.com/Chaoses-Ib/MarkupLanguages]`)
/// #example(`#a-badge(body: [Wikibooks])[https://en.wikibooks.org/wiki/LaTeX/Colors]`)
/// 
/// - uri (content):
/// - body (content):
#let a-badge(uri, body: none) = {
  import "www.typ"
  uri = util.to-string(uri)
  if body == none {
    let host = uri-host(uri).trim("www.", at: start)
    let badge = www.host-badge(host)
    if badge != none {
      return badge(uri)
    }
    
    body = host
  }
  www.badge-gray[
    #link(uri, body)
  ]
}
