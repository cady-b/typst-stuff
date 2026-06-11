// This will work in many cases but isn't guaranteed to; some `content` is impossible to transformed into a string (e.g. those retrieved from `context` blocks).
// In terms of attribution, one original author of such a function, according to https://sitandr.github.io/typst-examples-book/book/typstonomicon/extract_plain_text.html, is ntjess; the tools4typsts (t4t) package also has similar utilities. Many will have their own growing in their utils.
#let to-string(it, strict: true) = {
  let t = type(it)

  if it in (none, auto) {return ""}
  if t == str {return it}
  if t in (int, float, decimal) {return str(it)}
  if t == content {
    if it.has("text") {return it.text}
    if it.has("alt") {return to-string(it.alt)}
    if it.has("children") {return it.children.map(to-string).join()}
    if it.has("child") {return to-string(it.child)}
    if it.has("body") {return to-string(it.body)}

    let f = it.func()
    if f == [ ].func() {return " "}
    if f == smartquote {return if it.double {"\""} else {"'"}}
    if f == linebreak {return "\n"}
    if f == parbreak {return "\u{2029}"}
  }

  if not strict { return repr(it) }
  panic("Unexpected entry; type " + str(t) + " of " + repr(it))
}
