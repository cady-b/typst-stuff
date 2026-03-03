// we can't just warn, so we abuse an existing warning with our own data
#let _warn(body) = {
  let unknown-quantity = [#(label(body))]
}

#let db() = {
  import "data/quantities.typ": currency, quantities
  import "data/fix.typ": prefixes, suffixes

  (quantities + currency, prefixes, suffixes)
}

#let resolve(part, was_prefix, separator) = {
  let (db, prefixes, suffixes) = db()
  let unit = db.at(part, default: none)
  let prefix = prefixes.at(part, default: none)
  let suffix = suffixes.at(part, default: none)
  let sep = if not was_prefix { separator }

  if part == "per" {
    return (true, "/")
  }

  if unit != none {
    if type(unit) == dictionary and unit.at("weak", default: none) != none {
      return (true, sep + unit.at("weak"))
    } else {
      return (false, sep + unit)
    }
  }

  if prefix != none {
    return (true, sep + prefix)
  }

  if suffix != none {
    return (false, suffix)
  }

  _warn(part)
  return (was_prefix, none)
}

#let unit(body, separator: sym.space.sixth) = {
  let (db, prefixes, suffixes) = db()


  if type(body) == function {
    return (body)()
  }

  let was_prefix = true
  for part in body.split(regex("\s")) {
    let (p, b) = resolve(part, was_prefix, separator)
    was_prefix = p
    b
  }
}
