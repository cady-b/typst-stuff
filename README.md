<h1 align="center">Cady's Typst stuff</h1>

Built around this silly piece of (quite awesome) software: https://typst.app/open-source

## Packages

### null

> A thin, opinionated wrapper around [`zero`](https://typst.app/universe/package/zero), providing some functionality from the now unmaintained [`metro`](https://typst.app/universe/package/metro) package that I miss; because `zero.zi` wasn't quite what I needed for my documents (that is a great alternative for when this package starts to fall apart though).

## Community things

### Notable contributions to the Typst project

**[\#7423](https://redirect.github.com/typst/typst/pull/7423): Improve the font features interface**

> My first PR to `typst/typst`; niche spec compliance stuff I was nerd-sniped into by a user doing work with [Znamenny Notation](https://en.wikipedia.org/wiki/Znamenny_chant), which has a [pretty cool font](https://redirect.github.com/slavonic/Mezenets) that exposed a missing knob in Typst. More interesting (to me) than changing the type of a property however was discovering that pretty much no validation was done on raw OpenType features; I had a ton of fun trying to give as nice errors as possible within Typst while handling all the weirdness of Unicode and OT c:
>
> I learned a lot about Typst, notably the diagnostics model and how elements are defined. Beyond Typst, it was good practice of test coverage, correctness, and communication. It was the first time I had to pass a professional code review, which was a pretty interesting experience by itself; I'm very thankful to Laurenz for his patience and guidance with stylistic and technical considerations.

**[\#7528](https://redirect.github.com/typst/typst/pull/7528): Friendly hint and docs clarification for `array.sorted`**

> This one was quite silly: mere hours before the 0.14.1 release, a PR was crated and merged that changed the sorting algorithm to a more resilient one; this had the unfortunate side effect of exposing some previously fully invisible issues in user code as a hard error. I noticed this after upgrading and trying to compile my diary, which happened to be affected. Despite the error pointing right to the bad line, it was a quite pain to figure out why the behavior suddenly changed. It turned out that other users also experienced this issue, most notably [through the glossy package](https://redirect.github.com/swaits-typst-packages/glossy/issues/16).
>
> The PR isn't very interesting on a technical level, but it was an intense experience triaging and figuring out how best to handle the situation. I'm quite happy with the result and since had positive experiences helping people put the hint into action in the support channels. More recently, Laurenz talked talked about this among other considerations in defining and improving the stability of Typst and its ecosystem on his blog: https://laurmaedje.github.io/posts/evolving-typst

---

Things I've learned to dislike:
- SVG | \[link relevant issues here\]
