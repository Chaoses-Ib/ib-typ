/// CBOR ABI decorator.
#let abi-cbor(fn) = {
  if fn == none {
    return none
  }
  let f(..s) = cbor(fn(cbor.encode(s.named())))
  f
}

/// str-CBOR ABI decorator.
#let abi-str-cbor(fn) = {
  if fn == none {
    return none
  }
  let f(s) = cbor(fn(bytes(s)))
  f
}

#let plugin = plugin("ib_typ.wasm")

/// A fallible version of `plguin.{id}`.
/// 
/// - id (str):
/// -> function, none
#let plugin-at(id) = {
  dictionary(plugin).at(id, default: none)
}

/// A fallible version of `abi-cbor(plguin.{id})`.
/// 
/// - id (str):
/// -> function, none
#let plugin-abi-cbor(id) = {
  abi-cbor(plugin-at(id))
}

/// A fallible version of `abi-str-cbor(plguin.{id})`.
/// 
/// - id (str):
/// -> function, none
#let plugin-abi-str-cbor(id) = {
  abi-str-cbor(plugin-at(id))
}
