#let (value, qty, unit, zero) = {
  import "@preview/zero:0.5.0" as zero

  let _value = zero.num
  import "qty.typ": qty as _qty

  let _unit(value, qty, separator: sym.space.thin, unit-separator: auto, ..arg) = math.equation({
    _value(value, ..arg)
    separator
    _qty(qty, ..if unit-separator != auto { (separator: unit-separator) })
  })

  (_value, _qty, _unit, zero)
}
