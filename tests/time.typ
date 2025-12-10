#import "../src/lib.typ": time
#import time: *

= Date
#datetime_norm_rfc3339("251025")

#datetime_parse(251025)

#datetime_parse("251025")

== yymm
#datetime_parse(25, yymm: true) \
#datetime_parse(2510, yymm: true) \

= Date time
#datetime_norm_rfc3339("251025 00:26:00")

#datetime_norm_rfc3339("251025 0:26")
#datetime_norm_rfc3339("251025 00:26")

#datetime_parse("251025 00:26")

== Time
#datetime_norm_rfc3339("16:00")

#datetime_parse("16:00")

== Time offset
#datetime_norm_rfc3339("2025-10-25 00:26+08:00")

#datetime_parse("2025-10-25 00:26+08:00")

= RFC 3339
#datetime_parse("2025-10-25T00:26:00")

= Format
#t[25] \
#t[2510] \

#t[251025]
#t[2025-10-25]
#t("2025-10-25")

#t[251025 0:26]
#t[2025-10-25 00:26]
#t("2025-10-25 00:26")

#t[2025-10-25 00:26:00]
#t("2025-10-25 00:26:00")

#t("2025-01-01 01:02:03")

#t[16:00]
#t("16:00")

#t("2025-10-25 00:26", offset: -5)

#t("2025-10-25 00:26", offset: 8, body: "")

#t([2025-10-25 00:26], offset: 8, body: [CHANGE])
#t("2025-10-25 00:26", offset: 8, body: [CHANGE])
