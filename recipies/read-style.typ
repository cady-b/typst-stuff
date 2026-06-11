#let read-style(body, capture, callback) = {
  let styled = text(size: 1em, none).func()
  let scratch = state("scratch-pad")

  // store the relevant styles styles
  styled(context scratch.update(capture()), body.styles)

  // execute the callback with them
  context callback(scratch.get())
}

// --- Demo ---

#let p2 = {
  set par(hanging-indent: 2em)
  set text(red)

  par(lorem(50))
}

#read-style(p2, () => (par.hanging-indent, text.fill), ((indent, col)) => [
  #indent
  #col
])
