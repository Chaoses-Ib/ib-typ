/// - url (str):
/// - body (content):
#let wikipedia(url, body: [Wikipedia]) = [
  #import "badge.typ": *
  #badge-gray[#link(url, body)]
]

/// - url (str):
/// - body (content):
#let bangumi(url, body: [Bangumi]) = [
  #import "badge.typ": *
  #badge-red[#link(url, body)]
]