#import "../ffi.typ": plugin-abi-cbor, plugin-abi-str-cbor
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
