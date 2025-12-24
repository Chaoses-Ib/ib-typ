/// - Can only be used within `context`
///   unless `body` is a str or raw block with line breaks.
/// 
/// - body (content, str):
#let is-multiline(body) = {
  if type(body) == str and body.contains("\n") {
    return true
  }
  if type(body) == content and body.func() == raw {
    return is-multiline(body.text)
  }
  measure(body).height >= measure([~\ ~]).height
}

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
