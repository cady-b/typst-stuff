#let (value, qty, unit, zero) = {
  import "@preview/zero:0.5.0" as zero

  let _value = zero.num;
  import "qty.typ": qty as _qty

  let _unit(value, qty, separator: sym.space.thin) = math.equation({
    _value(value)
    separator
    _qty(qty)
  })

  (_value, _qty, _unit, zero)
}
