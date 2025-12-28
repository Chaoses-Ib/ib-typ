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

/// If `it` is a raw block with optional surrounding spaces,
/// return its text; otherwise, return `it`.
/// 
/// Usage:
/// - Accept raw block as string to avoid explicit line breaks (`\`).
#let to-string-if-raw(
  it,
  trim: true,
) = {
  let map = if trim {
    str.trim
  } else {
    i => i
  }
  if type(it) == content {
    if it.func() == raw {
      map(it.text)
    } else if it.has("children") {
      let children = it.children
      let raws = children.filter(i => i.func() == raw)
      if raws.len() != 1 {
        return it
      }
      let spaces = children.filter(i => i == [ ]).len()
      if raws.len() + spaces != children.len() {
        return it
      }
      map(raws.pop().text)
    } else {
      it
    }
  } else {
    it
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

/// Usage:
/// - Implement show-if: `#show: to-function(show-f-or-none)`
/// 
/// - v (function, none):
/// -> function
#let to-function(
  v,
  default: v => v,
) = {
  let t = type(v)
  if t == function {
    v
  } else {
    default
  }
}
