/// - s (str, int):
/// -> s
#let datetime_norm_rfc3339(s) = {
  if type(s) == int {
    s = str(s)
  }
  s.replace(regex(`^(\d\d)(\d\d)(\d\d)`.text),
    // 20$1-$2-$3
    m => "20" + m.captures.at(0) + "-" + m.captures.at(1) + "-" + m.captures.at(2)
  )
  .replace(regex(`^\d\d:`.text), m => "0000-01-01T" + m.text)
  .replace(" ", "T")
  // .replace(regex(`[^:]\d\d:\d\d$`.text), m => m.text + ":00")
  .replace(regex(`([^:+-]\d\d:\d\d)($|[Z+-])`.text), m => m.captures.at(0) + ":00" + m.captures.at(1))
}

/// - s (str, int):
/// -> datetime
#let datetime_parse(s) = {
  // TODO: https://github.com/typst/typst/issues/4107
  toml(bytes("d=" + datetime_norm_rfc3339(s))).d
}

/// - d (datetime):
/// - offset (none, int, str): Time offset
/// -> str
#let datetime_format(d, offset: none) = {
  if type(offset) == int {
    offset = if offset > 0 {
      "+" + str(offset)
    } else {
      str(offset)
    }
  }
  if d.minute() == none {
    d.display("[year]-[month]-[day]")
  } else if d.year() == 0  {
    d.display("[hour]:[minute]")
  } else {
    d.display("[year]-[month]-[day] [hour]:[minute]")
  } + offset
}

#import "@preview/badgery:0.1.1"

/// #example(`#t(251025)`)
/// #example(`#t("251025")`)
/// #example(`#t("16:00")`)
/// #example(`#t("251025 00:26")`)
/// #example(`#t("2025-10-25 00:26")`)
/// #example(`#t("2025-10-25T00:26:00")`)
/// #example(`#t("2025-10-25 00:26", offset: -5)`)
/// #example(`#t("2025-10-25 00:26", offset: 8, body: [CHANGE])`)
/// 
/// - s (str, int):
/// - offset (none, int, str): Time offset
/// - body (none, str, content):
#let t(s, offset: none, body: none) = {
  badgery.badge-gray([
    #datetime_format(datetime_parse(s), offset: offset) #body
  ])
}
