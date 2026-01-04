#import "../src/color.typ": *
#import "../src/texts.typ": *
#import "../src/util.typ": *

= #quote-cjk-line-break
#dbg[「勝ち取るんだ最高の学園生活を！」
「君に恋をしてしまったんだ…」
「待って！　友達どこいった！？」]

#dbg[「勝ち取るんだ最高の学園生活を！」
#gold[「君に恋をしてしまったんだ…」]
「待って！　友達どこいった！？」]

#dbg(quote-cjk-line-break[「勝ち取るんだ最高の学園生活を！」
#gold[「君に恋をしてしまったんだ…」]
「待って！　友達どこいった！？」])

#show: quote-cjk-line-break

#show: it => {
  if it == [ ] {
    return [fck]
  }
  it
}

「勝ち取るんだ最高の学園生活を！」
「君に恋をしてしまったんだ…」
「待って！　友達どこいった！？」

「勝ち取るんだ最高の学園生活を！」
#gold[「君に恋をしてしまったんだ…」]
「待って！　友達どこいった！？」
