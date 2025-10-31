#import "badge.typ": *

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
#let bangumi(url, body: [Bangumi]) = [
  #badge-red[#link(url, body)]
]