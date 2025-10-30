/// - color (color):
/// - body (content):
#let dye(color, body) = {
  set text(color)
  body
}

/// OneNote colors
#let one = (
  Red: rgb("#ff0000"),
  red: rgb("#e84c22"),
  salmon: rgb("#ff513e"),
  orange: rgb("#ed7d31"),
  green: rgb("#00b050"),
  blue: rgb("#00b0f0"),
  purple: rgb("#8064a2"),
  Purple: rgb("#7030a0"),
  gray: rgb("#757070"),
)
#let Red = dye.with(one.Red)
#let red = dye.with(one.red)
#let salmon = dye.with(one.salmon)
#let orange = dye.with(one.orange)
#let green = dye.with(one.green)
#let blue = dye.with(one.blue)
#let purple = dye.with(one.purple)
#let Purple = dye.with(one.Purple)
#let gray = dye.with(one.gray)

#let color = color
#import "@preview/splash:0.5.0": *
