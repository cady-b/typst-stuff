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

  if it in (none, auto) { return "" }
  if t == str { return it }
  if t in (int, float, decimal) { return str(it) }
  if t == content {
    if use-alt and it.has("alt") { return to-string(it.alt) }

    if it.has("text") { return it.text }
    if it.has("children") { return it.children.map(to-string).join() }
    if it.has("child") { return to-string(it.child) }
    if it.has("body") { return to-string(it.body) }

    let f = it.func()
    if f == [ ].func() { return " " }
    if f == smartquote { return if it.double { "\"" } else { "'" } }
    if f == linebreak { return "\n" }
    if f == parbreak { return "\u{2029}" }
  }

  if use-repr { return repr(it) }
  panic("Unexpected entry; type " + str(t) + " of " + repr(it))
}


// --- Tests ---

#let test(a, b, func: to-string) = assert.eq(a, func(b))

#test("", none)
#test("", auto)
#test("a", "a")
#test("1", 1)
#test("1.5", 1.5)
#test("3.1415926535897932384626433832", decimal("3.1415926535897932384626433832"))

#test("The square root of x", math.equation(alt: "The square root of x", $sqrt(x)$))
#test("foo bar buzz")[foo bar buzz]
#test("foo bar buzz")[foo bar #rect[buzz]]
#test("foo bar buzz")[foo #rect[bar #rect[buzz]]]
// TODO: `child` & more complex content

#test(" ")[ ]
#test("\"quote\"")["quote"]
#test("'quote'")['quote']
#test("a\n b")[a\ b]
#test("a\u{2029}b")[a#parbreak()b]

#test("context()", context {}, func: to-string.with(use-repr: true))
#test("x", math.equation(alt: "nope", $x$), func: to-string.with(use-alt: false))
