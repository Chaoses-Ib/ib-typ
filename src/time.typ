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
  .replace(" ", "T")
  .replace(regex(`[^:]\d\d:\d\d$`.text), m => m.text + ":00")
}

/// - s (str, int):
/// -> datetime
#let datetime_parse(s) = {
  // TODO: https://github.com/typst/typst/issues/4107
  toml(bytes("d=" + datetime_norm_rfc3339(s))).d
}

/// - d (datetime):
/// -> str
#let datetime_format(d) = {
  if d.minute() == none {
    d.display("[year]-[month]-[day]")
  } else {
    d.display("[year]-[month]-[day] [hour]:[minute]")
  }
}

#import "@preview/badgery:0.1.1"

/// #example(`t(251025)`)
/// #example(`t("251025")`)
/// #example(`t("251025 00:26")`)
/// #example(`t("2025-10-25 00:26")`)
/// #example(`t("2025-10-25T00:26:00")`)
/// 
/// - s (str, int):
#let t(s) = badgery.badge-gray(datetime_format(datetime_parse(s)))
