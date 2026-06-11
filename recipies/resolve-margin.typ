#let margin() = {

  // --- helpers ---

  let resolve-page = {
    let (height, width) = (page.height, page.width)

    // if one is auto, use the other
    if height == auto { height = width }
    if width == auto { width = height }

    // if neither is given, default to A4: https://github.com/typst/typst/blob/de6f4009/crates/typst-layout/src/pages/run.rs#L115
    if (height, width) == (auto, auto) {
      height = 297.0mm
      width = 210.0mm
    }

    (height: height, width: width)
  }

  let margin = page.margin
  let auto-margin = 2.5 / 21 * calc.min(..resolve-page.values()) // as per https://typst.app/docs/reference/layout/page/#parameters-margin
  let resolve-rel(rel, length) = { rel.ratio * length + rel.length }

  // --- actual evaluation ---

  if margin == auto {
    return (top: auto-margin, right: auto-margin, bottom: auto-margin, left: auto-margin)
  }

  if type(margin) == relative {
    let vertical = resolve-rel(margin, resolve-page.height)
    let horizontal = resolve-rel(margin, resolve-page.width)

    return (top: vertical, right: horizontal, bottom: vertical, left: horizontal)
  }

  if type(margin) == dictionary {
    let (height: ph, width: pw) = resolve-page
    let rest = margin.at("rest", default: auto-margin)

    let make-side(var, alt, rel) = {
      let var = margin.at(var, default: auto)
      if var == auto {
        margin.at(alt, default: auto-margin)
      } else {
        resolve-rel(var, rel)
      }
    }

    let top = make-side("top", "y", ph)
    let bottom = make-side("bottom", "y", ph)
    let left = make-side("left", "x", pw)
    let right = make-side("right", "x", pw)

    // handle binding (Typst would panic if there are conflicts with left / right, so we can unconditionally overwrite)

    let binding = calc.even(here().page())
    let has(v) = not margin.at(v, default: none) in (none, auto)

    if has("inside") {
      let v = resolve-rel(margin.inside, resolve-page.width)
      if not binding { left = v } else { right = v }
    }

    if has("outside") {
      let v = resolve-rel(margin.outside, resolve-page.width)
      if binding { left = v } else { right = v }
    }

    return (top: top, right: right, bottom: bottom, left: left)
  }

  panic("unknown configuration")
}

// --- Test ---

#let p = page.with(
  height: 8cm,
  width: 8cm,
  foreground: context block(fill: white.transparentize(20%), outset: 1em, [#margin()]),
  background: context {
    let (top, right, bottom, left) = margin()
    let lots = 10000pt
    set rect(stroke: green)

    place(std.left, dx: left, rect(height: lots, width: 0pt))
    place(std.right, dx: -right, rect(height: lots, width: 0pt))
    place(std.top, dy: top, rect(height: 0pt, width: lots))
    place(std.bottom, dy: -bottom, rect(height: 0pt, width: lots))
  },
  place(par(justify: true, lorem(200))),
)

#p(margin: auto)
#p(margin: 10pt)
#p(margin: 20%)
#p(margin: (top: 10pt))
#p(height: auto, margin: auto)
#p(height: auto, width: auto, margin: auto)
#p(margin: (inside: 10pt))
#p(margin: (outside: 10pt))
