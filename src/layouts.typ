/// Block with slots.
#let block-slot(
  header: none,
  body,
  ..args
) = {
  block(
    inset: (top: 0pt, rest: 4pt),
    fill: luma(245),
    radius: 2pt,
    stroke: luma(235),
    ..args,
    {
      {
        move(
          header,
          dx: -4pt,
          dy: 0pt
        )
      }
      block(
        body,
        above: 4pt,
        below: 0pt,
      )
    }
  )
}
