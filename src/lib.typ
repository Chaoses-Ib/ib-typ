#import "@preview/cmarker:0.1.6": render as md

#import "badge.typ": *
#import "code.typ"
#import "color.typ"
#import "color.typ": (
  Red, red,
  salmon,
  orange,
  gold, yellow,
  green, green-light, 
  blue,
  purple-light, purple, Purple,
  gray,
  black,
  white,
)
#import "icon.typ"
#import "link.typ" as links
#import "link.typ": (
  a,
  a-badge,
)
#import "list.typ" as lists
#import "time.typ"
#import "time.typ": t
#import "uri.typ"
#import "util.typ"
#import "www.typ": (
  ai,
  wikipedia,
  github,
  steam,
  dlsite,
  bangumi,
)

#let ib(it) = {
  show: lists.checklist
  it
}
