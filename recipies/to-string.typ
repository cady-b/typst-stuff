// This will work in many cases but isn't guaranteed to; some `content` doesn't
// have a textual meaning or is impossible to turn back into a string (e.g.
// those retrieved from `context`). You'll likely wish to customize it for your
// specific use case.
//
// Maintaining a line of attribution is difficult for such an ubiquitous routine
// but according to https://sitandr.github.io/typst-examples-book/book/typstonomicon/extract_plain_text.html
// one original author of such a function is ntjess; the tools4typsts package
// (`t4t` on Universe) also has similar utilities. This version is inspired by
// the many questions about stringification posted to the Typst support channels
// and currently maintained by me (Liara).
#let to-string(it, use-alt: true, use-repr: false) = {
  let t = type(it)

  if it in (none, auto) {return ""}
  if t == str {return it}
  if t in (int, float, decimal) {return str(it)}
  if t == content {
    if it.has("text") {return it.text}
    if use-alt and it.has("alt") {return to-string(it.alt)}
    if it.has("children") {return it.children.map(to-string).join()}
    if it.has("child") {return to-string(it.child)}
    if it.has("body") {return to-string(it.body)}

    let f = it.func()
    if f == [ ].func() {return " "}
    if f == smartquote {return if it.double {"\""} else {"'"}}
    if f == linebreak {return "\n"}
    if f == parbreak {return "\u{2029}"}
  }

  if use-repr { return repr(it) }
  panic("Unexpected entry; type " + str(t) + " of " + repr(it))
}
