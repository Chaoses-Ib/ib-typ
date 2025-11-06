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
