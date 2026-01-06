/// -> content, none
#let query-heading-current() = {
  query(heading.where()
    .before(here()))
    .last(default: none)
}

/// -> content, none
#let query-heading-next(h) = {
  query(heading.where(level: h.level)
    .after(h.location(), inclusive: false))
    .first(default: none)
}

/// Modify `sel` to select within the scope of current heading
/// (i.e. after last heading, before next heading of the same level).
/// 
/// - sel (str, regex, label, selector, location, function):
/// -> selector
#let selector-scope(target) = {
  let sel = selector(target)
  let current = query-heading-current()
  if current != none {
    sel = sel.after(current.location(), inclusive: false)

    let next = query-heading-next(current)
    if next != none {
      sel = sel.before(next.location(), inclusive: false)
    }
  }
  sel
}

/// Query `target` within the scope of current heading
/// (i.e. after last heading, before next heading of the same level).
/// 
/// - sel (str, regex, label, selector, location, function):
/// -> array
#let query-scope(target) = {
  query(selector-scope(target))
}
