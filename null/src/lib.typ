#let (value, unit, qty, zero) = {
  import "@preview/zero:0.5.0" as zero
  import "unit.typ": unit

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
 
  // generate alt text for quantities
  let qty-alt(alt, value, qty) = {
    if type(alt) == str { return alt; }

    let alt = value-alt(alt, value)
    alt += " "
    if type(qty) == str { alt += qty }
    else if type(qty) == function and type(qty()) == str { alt += qty() }
    else {
      let unable-to-produce-alt-text-from = [#(label(repr(qty)))]
    }

    alt
  }

  // --------------
  // | main funcs |
  // --------------

  let value(alt: none, number, ..args) = math.equation(alt: value-alt(alt, number), zero.num(number, ..args))

  let qty(alt: none, value, qty, separator: sym.space.narrow.nobreak, unit-separator: auto, ..arg) = math.equation(alt: qty-alt(alt, value, qty), {
    zero.num(value, ..arg)
    separator
    unit(qty, ..if unit-separator != auto { (separator: unit-separator) })
  })

  // export
  (value, unit, qty, zero)
}
