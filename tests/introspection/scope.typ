#import "../../src/introspection/scope.typ": *

#context query-scope(<test>)

= a
#context query-scope(<test>)

a <test>

#context query-scope(<test>)

== a.a
#context query-scope(<test>)

a.a <test>

== a.b
#context query-scope(<test>)

a.b <test>

= b
#context query-scope(<test>)

b <test>
