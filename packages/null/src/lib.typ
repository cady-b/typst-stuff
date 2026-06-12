#let (value, unit, qty, with-db, zero) = {
  import "@preview/zero:0.5.0" as zero
  import "unit.typ": _unit

  // generate alt text for values
  let value-alt(alt, value) = {
    if type(alt) == str { return alt; }

    let alt = ""
    if type(value) == str { alt += value }
    else if type(value) in (int, float, decimal) { alt += str(value) }
    else if type(value) == content {
      if value.has("text") { alt += value.text }
      else { let unable-to-produce-alt-text-from = [#(label(repr(value) + " " + repr(value.fields())))] }
    } else {
      let unable-to-produce-alt-text-from = [#(label(repr(value)))]
    }

    alt
  }

  let unit-alt(alt, unit) = {
    if type(alt) == str { return alt; }
    if type(unit) == str { return unit; }
    if type(unit) == function and type(unit()) == str { return unit(); }

    let unable-to-produce-alt-text-from = [#(label(repr(qty)))]
  }
 
  // generate alt text for quantities
  let qty-alt(alt, value, qty) = {
    if type(alt) == str { return alt; }

    let alt = value-alt(none, value)
    alt += " "
    alt += unit-alt(none, qty)

    alt
  }

  let value(alt: none, number, ..args) = math.equation(alt: value-alt(alt, number), zero.num(number, ..args))

  let with-db(db: (:), prefixes: (:), suffixes: (:)) = {
    let db-extra = (db, prefixes, suffixes)

    let unit(alt: none, body, separator: sym.space.sixth) = math.equation(
      alt: unit-alt(alt, body),
      _unit(db-extra, body, separator)
    )

    let qty(alt: none, value, qty, separator: sym.space.narrow.nobreak, unit-separator: sym.space.sixth, ..arg) = math.equation(
      alt: qty-alt(alt, value, qty),
      {
        zero.num(value, ..arg)
        separator
        _unit(db-extra, qty, unit-separator)
      }
    )

    (unit: unit, qty: qty)
  }

  // export
  (value: value, ..with-db(), with-db: with-db, zero: zero)
}
