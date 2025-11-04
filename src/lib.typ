#import "@preview/cmarker:0.1.6": render as md

#import "badge.typ": *
#import "code.typ"
#import "color.typ"
#import "color.typ": Red, red, salmon, orange, green, blue, purple, Purple, gray
#import "color.typ": gold, yellow
#import "color.typ": green-light
#import "color.typ": purple-light
#import "list.typ" as lists
#import "time.typ"
#import "time.typ": t
#import "www.typ": *

#let ib(it) = {
  show: lists.checklist
  it
}
