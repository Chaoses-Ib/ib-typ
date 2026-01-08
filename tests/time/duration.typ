#import "../../src/time/duration.typ": *

`1.1h4+90`:
#duration-eval-format("1.1h4+90")

#show: duration-suffix
- a|b
- a |b \
- Activity  |42
- Strong activity  |123

= Aggregate
#import "../../src/time/mod.typ": t

#show: heading-aggregate-t-duration
== No t

== One t
#t[0:10]

== t
#t[0:10]

#t[0:20] #t[0:30]

#t[0:40]
