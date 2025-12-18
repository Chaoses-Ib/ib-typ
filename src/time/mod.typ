#import "../util.typ": to-string
#import "../layouts.typ"

#import "duration.typ": *

/// - s (str, int, content):
/// -> s
#let datetime_norm_rfc3339(s) = {
  // if type(s) == int {
  //   s = str(s)
  // }
  s = to-string(s)
  s.replace(regex(`^(\d\d)(\d\d)(\d\d)`.text),
    // 20$1-$2-$3
    m => "20" + m.captures.at(0) + "-" + m.captures.at(1) + "-" + m.captures.at(2)
  )
  // Prefix leading zero to hour
  .replace(regex(`(^|[^\d:+-])(\d:)`.text), m => m.captures.at(0) + "0" + m.captures.at(1))
  .replace(regex(`^\d\d:`.text), m => "0000-01-01T" + m.text)
  .replace(" ", "T")
  // .replace(regex(`[^:]\d\d:\d\d$`.text), m => m.text + ":00")
  .replace(regex(`([^:+-]\d\d:\d\d)($|[Z+-])`.text), m => m.captures.at(0) + ":00" + m.captures.at(1))
}

/// - s (str, int, content):
/// -> datetime
#let datetime_parse(s, yymm: false) = {
  s = to-string(s)
  if yymm {
    let d = int(s)
    if s.len() == 2 {
      return datetime(year: 2000 + d, month: 1, day: 1)
    } else if s.len() == 4 {
      return datetime(
        year: 2000 + calc.div-euclid(d, 100),
        month: calc.rem(d, 100),
        day: 1
      )
    }
  }
  // TODO: https://github.com/typst/typst/issues/4107
  toml(bytes("d=" + datetime_norm_rfc3339(s))).d
}

/// - d (datetime, int, str, content):
/// - offset (none, int, str): Time offset
/// - f(str): Preferred (short) date format
/// -> str
#let datetime_format(
  d,
  offset: none,
  f: "mmdd",
) = {
  if type(d) == int {
    let r100 = calc.rem(d, 100)
    // Prefer mmdd date
    if (f == "mmdd" and d < 1300 and (d >= 100 or d <= 31)
      or r100 > 12
    ) {
      // mmdd date
      if d < 100 {
        return datetime(year: 0, month: 1, day: d)
          .display("[day]")
      } else if d < 10000 {
        return datetime(
          year: 0,
          month: calc.div-euclid(d, 100),
          day: r100
        ).display("[month]-[day]")
      }
    }

    // yymm date
    if d < 100 {
      return datetime(year: 2000 + d, month: 1, day: 1)
        .display("[year]")
    } else if d < 10000 {
      return datetime(
        year: 2000 + calc.div-euclid(d, 100),
        month: calc.rem(d, 100),
        day: 1
      ).display("[year]-[month]")
    }
  } else if type(d) != datetime {
    d = to-string(d)

    // Short date with time
    let m = d.match(regex(`^(?:\d\d|\d{4}) `.text))
    if m != none {
      let date = m.text.slice(0, -1)
      let time = d.slice(m.end)
      return datetime_format(date) + " " + datetime_format(time)
    }

    // May be int
    return datetime_format(datetime_parse(d))
  }

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

/// Time stamp/mark.
/// 
/// == Content style
/// #example(`#t[25]`)
/// #example(`#t[2510]`)
/// #example(`#t[251025]`)
/// #example(`#t[16:00]`)
/// #example(`#t[1025 0:26]`)
/// #example(`#t[251025 0:26]`)
/// #example(`#t[251025 00:26]`)
/// #example(`#t[2025-10-25 00:26]`)
/// #example(`#t[2025-10-25T00:26:00]`)
/// 
/// == String style
/// #example(`#t(251025)`)
/// #example(`#t("251025")`)
/// #example(`#t("16:00")`)
/// #example(`#t("251025 00:26")`)
/// #example(`#t("2025-10-25 00:26")`)
/// #example(`#t("2025-10-25T00:26:00")`)
/// 
/// == With args
/// #example(`#t([2025-10-25 00:26], offset: -5)`)
/// #example(`#t("2025-10-25 00:26", offset: -5)`)
/// #example(`#t("2025-10-25 00:26", offset: 8, body: [CHANGE])`)
/// 
/// - s (str, int, content):
/// - offset (none, int, str): Time offset
/// - body (none, str, content):
#let t(s, offset: none, body: none) = {
  badgery.badge-gray([
    #datetime_format(s, offset: offset) #body
  ])
}

/// Time block for non-linear time series.
/// 
/// #example(`#t-block[251214][test]`)
/// #example(`#t-block(fill: yellow)[251214][test]`)
#let t-block(
  s,
  offset: none,
  header: none,
  body,
  ..args
) = {
  layouts.block-slot(
    header: t(s, offset: offset, body: header),
    body,
    fill: luma(245),
    radius: 2pt,
    stroke: luma(235),
    ..args
  )
}
