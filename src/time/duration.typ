#import "../util.typ": to-string, plugin
#import "../color.typ"

#let duration-eval-format(s) = cbor(plugin.duration_eval_format(bytes(s)))

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
