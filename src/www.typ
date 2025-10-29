/// - url (str):
#let bangumi(url) = [
  #import "badge.typ": *
  #badge-red[#link(url)[Bangumi]]
]