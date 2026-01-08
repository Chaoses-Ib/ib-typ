#import "../ffi.typ": plugin-abi-cbor, plugin-abi-str-cbor
#import "../introspection/scope.typ": query-scope
#import "../util.typ": to-string
#import "../color.typ"

#let duration-eval-format = plugin-abi-str-cbor("duration_eval_format")

#let duration-suffix(
  it,
  strong: true,
) = {
  show regex(`^(?:.+\s)?\|\S+$`.text): it => {
    let parts = to-string(it).split("|")
    let d = parts.pop()
    let s = parts.join("|")

    let r = duration-eval-format(d)

    let d = sym.space.nobreak + "|" + r.s
    d = color.gray(d)
    if strong and r.seconds >= 2 * 60 * 60 {
      d = std.strong(d)
      s = std.strong(s)
    }
    s + d
  }
  it
}

#let times-to-duration-and-eval = plugin-abi-cbor("times_to_duration_and_eval")

/// Aggregate `t()` and show times (and duration).
#let aggregate-t-duration(it) = {
  it
  context {
    let times = query-scope(<ib.time.t>)
    let r = times-to-duration-and-eval(times: times.map(m => m.value))
    if r.s.len() == 0 {
      return
    }

    import "../badge.typ"
    set text(size: 10pt, weight: "medium")
    block(badge.badge-gray(
      r.s
    ), above: 0pt, below: 4pt)
  }
}

/// Aggregate `t()` and show times (and duration) under heading and title.
#let heading-aggregate-t-duration(it) = {
  show title: aggregate-t-duration
  show heading: aggregate-t-duration
  it
}
