#import "../src/refs.typ": *

= Quote
#q[#lorem(5)]

A#quote[
#`「私は君を喰べに来ました。」

突如現れた人魚の少女・汐莉は
海辺の街に独り暮らす比名子の手を取り、優しく語りかける。
`.text]B

A#q[`
「私は君を喰べに来ました。」

突如現れた人魚の少女・汐莉は
海辺の街に独り暮らす比名子の手を取り、優しく語りかける。
`]B

A#q[
「私は君を喰べに来ました。」

突如現れた人魚の少女・汐莉は \
海辺の街に独り暮らす比名子の手を取り、優しく語りかける。
]B

A#quote[
- 1
- 2
]B

A#q[
- A
- B
]B

== Attribution
#quote(attribution: [サグメ])[#lorem(5)]

#{
  show: quote-inline-attribution 
  quote(attribution: [サグメ])[#lorem(5)]
} \
#q(at: [サグメ])[#lorem(5)] \
#q-at[サグメ][#lorem(5)] \

#q-at[][#lorem(5)]

#q-i[#lorem(5)]
