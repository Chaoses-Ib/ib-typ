#import "badge.typ": *
#import "icon.typ"
#import "uri.typ": host-ancestors

/// - body (content):
#let ai(body: [AI]) = [
  #badge-gray(body)
]

/// - url (str):
/// - body (content):
#let wikipedia(url, body: [Wikipedia]) = [
  #badge-gray[#link(url, body)]
]

/// - url (str):
/// - body (content):
#let github(url, body: [GitHub]) = {
  badge-gray[
    #box(icon.simple("github"))
    #link(url, body)
  ]
}

/// - url (str):
/// - body (content):
#let steam(url, body: [Steam]) = [
  #badge-blue[#link(url, body)]
]

/// - url (str):
/// - body (content):
#let dlsite(url, body: [DLsite]) = [
  #badge-blue[#link(url, body)]
]

/// - url (str):
/// - body (content):
#let bangumi(url, body: [Bangumi]) = [
  #badge-red[#link(url, body)]
]

#let HOST-BADGE = (
  "wikipedia.org": wikipedia,
  "github.com": github,
  "steampowered.com": steam,
  "dlsite.com": dlsite,
  "bangumi.tv": bangumi,
  "bgm.tv": bangumi,
)

/// - host (str):
/// -> function, none
#let host-badge(host) = {
  for host in host-ancestors(host) {
    if host in HOST-BADGE {
      return HOST-BADGE.at(host)
    }
  }
} 
