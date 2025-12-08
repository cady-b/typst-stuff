#let (value, unit, qty, zero) = {
  import "@preview/zero:0.5.0" as zero

  let _value = zero.num
  import "unit.typ": unit as _unit

  let _qty(value, qty, separator: sym.space.thin, unit-separator: auto, ..arg) = math.equation({
    _value(value, ..arg)
    separator
    _unit(qty, ..if unit-separator != auto { (separator: unit-separator) })
  })

  (_value, _unit, _qty, zero)
}
