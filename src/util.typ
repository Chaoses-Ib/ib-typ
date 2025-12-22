#let plugin = plugin("ib_typ.wasm")

/// Usage:
/// - Trailing `str` arguments
/// 
/// #example(`#to-string[This _*cool*_ project]`)
/// #example(`#to-string[Hello $sin x$ World]`)
#let to-string(it) = {
  // https://github.com/typst/typst/issues/2196#issuecomment-1728135476
  if type(it) == str {
    it
  } else if type(it) != content {
    str(it)
  } else if it.has("text") {
    // https://github.com/typst/typst/discussions/3876
    // if type(it.text) == str {
    //   it.text
    // } else {
    //   to-string(it.text)
    // }
    it.text
  } else if it.has("children") {
    it.children.map(to-string).join()
  } else if it.has("body") {
    to-string(it.body)
  } else if it == [ ] {
    " "
  }
}

/// - v (int, bool, float, decimal, str, content):
/// -> int, none
#let to-int(
  it,
  default: none,
) = {
  let t = type(it)
  if t == int {
    it
  } else if t == bool or t == float or t == decimal {
    int(it)
  } else if t == datetime {
    default
  } else if t == str {
    if it.contains(regex(`^[0-9]+$`.text)) {
      int(it)
    } else {
      default
    }
  } else {
    to-int(to-string(it))
  }
}
