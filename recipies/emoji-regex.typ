// don't ask (https://typst.app/project/rLLcwjnMc1YbSo514X8aor)
#show regex({
  let emoji = {
    "("
      "\p{Emoji}("
        "\p{EMod}|"                       // Emoji modifiers
        "\x{FE0F}\x{20E3}?|"              // Keycap sequences
        "[\x{E0020}-\x{E007E}]+\x{E007F}" // Tag sequences
      ")|"
      "[\p{Emoji}&&\p{Other_symbol}]"     // Single char emojis
    ")"
  }
  // Flag emojis, normal emojis, zero-width-joiner sequences
  "\p{RI}{2}|" + emoji + "(?:\x{200D}" + emoji + ")*"
  // check for the text-representation selector
  "\x{FE0E}?"
}): it => {
  if it.text.codepoints().last(default: "") == "\u{fe0e}" { return it; }
  text(font: scheme.font.emoji, size: .8em, top-edge: .6em, it)
}
