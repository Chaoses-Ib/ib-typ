/// - color (color):
/// - body (content):
#let dye(color, body) = {
  set text(color)
  body
}

/// OneNote-based colors
/// - Readability
#let one = (
  Red: rgb("#ff0000"),
  red: rgb("#e84c22"),
  salmon-450: rgb("#ff513e"),
  salmon: rgb("#fa8072"),
  orange: rgb("#ed7d31"),
  gold: rgb("#ffc000"),
  green: rgb("#00b050"),
  green-light: rgb("#90d090"),
  blue: rgb("#00b0f0"),
  purple-light: rgb("#817FA9"),
  purple: rgb("#8064a2"),
  Purple: rgb("#7030a0"),
  gray: rgb("#757070"),
)
#let Red = dye.with(one.Red)
#let red = dye.with(one.red)
#let salmon = dye.with(one.salmon)
#let orange = dye.with(one.orange)
#let gold = dye.with(one.gold)
#let green = dye.with(one.green)
#let green-light = dye.with(one.green-light)
#let blue = dye.with(one.blue)
#let purple-light = dye.with(one.purple-light)
#let purple = dye.with(one.purple)
#let Purple = dye.with(one.Purple)
#let gray = dye.with(one.gray)

#let color = color
#import "@preview/splash:0.5.0": *

#let yellow = dye.with(tailwind.yellow-400)
