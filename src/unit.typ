// we can't just warn, so we abuse an existing warning with our own data
#let _warn(body) = {
  let unknown-quantity = [#(label(body))]
}

#let db() = {
  import "data/quantities.typ": currency, quantities
  import "data/fix.typ": prefixes, suffixes

  (quantities + currency, prefixes, suffixes)
}

#let unit(body, separator: sym.space.sixth) = {
  let (db, prefixes, suffixes) = db()

  let was_prefix = true
  for part in body.split(regex("\s")) {
    let unit = db.at(part, default: none)

    if unit == none {
      let prefix = prefixes.at(part, default: none)

      if prefix == none {
        let suffix = suffixes.at(part, default: none)

        if suffix == none {
          _warn(part)
        } else {
          was_prefix = false
          suffix
        }
      } else {
        if not was_prefix {
          separator
        }
        was_prefix = true
        prefix
      }
    } else {
      if not was_prefix {
        separator
      }

      if type(unit) == dictionary and unit.at("weak", default: none) != none {
        was_prefix = true
        unit.at("weak")
      } else {
        was_prefix = false
        unit
      }
    }
  }
}
