/// #link("https://simpleicons.org/")[Simple Icons]
#let simple(id, height: 0.9em, color: "default") = {
  import "@preview/sicons:15.13.0" as simple
  box(image(simple.p.simple_icons_slug_colored(bytes(id), bytes(color)), height: height))
}
