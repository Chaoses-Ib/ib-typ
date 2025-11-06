/// - uri (str):
/// -> str, none
#let uri-host(uri) = {
  let m = uri.find(regex(`//[^/?#]+`.text))
  if m != none {
    m.slice(2)
  }
}

/// - host (str):
/// -> array
#let host-ancestors(host) = {
  let ancestors = (host,)
  let i = host.position(".")
  while i != none {
    host = host.slice(i + 1)
    ancestors.push(host)
    i = host.position(".")
  }
  ancestors
}
