# Sjisgrep - Binary grep with native support for searching in different Japanese encodings.

Sjisgrep is an enhanced binary string searcher which uses the [Boyer-Moore string-search algorithm](https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_string-search_algorithm) and multiple Japanese encodings ([Shift JIS](https://en.wikipedia.org/wiki/Shift_JIS), [EUC-JP](https://en.wikipedia.org/wiki/Extended_Unix_Code#EUC-JP) and [ISO-2022-JP](https://en.wikipedia.org/wiki/ISO/IEC_2022)) to find Japanese text in a binary file, e.g. video game ROMs.

## How to use

Build the application and run `sjisgrep` followed by its arguments on the terminal.

Output of `sjisgrep --help`:

```
Binary grep with native support for searching in different Japanese encodings

Usage: sjisgrep [OPTIONS] <PATTERN> <FILE> [ENCODING]

Arguments:
  <PATTERN>   Pattern that will be searched in the binary
  <FILE>      Path of the binary
  [ENCODING]  Encoding of the pattern [default: shift-jis] [possible values: shift-jis, euc-jp, iso2022-jp]

Options:
  -b, --beginning  Return the address related to the first character of the string found, not the pattern
  -h, --help       Print help information
  -V, --version    Print version information
```

## Examples

### Search for text in a ROM:

Source: [Atelier Marie: The Alchemist of Salburg](https://en.wikipedia.org/wiki/Atelier_Marie:_The_Alchemist_of_Salburg).

![Marie no Atelier](static/marie_no_atelier.jpg)

```
$ sjisgrep 参ったなぁ Marie\ no\ Atelier\ -\ Salburg\ no\ Renkinjutsushi\ \(Japan\)\ \(Track\ 1\).bin
0x1BEB0 "（…夢かぁ。参ったなぁ）"
```

The address returned (`0x1BEB0`) is the address of the first character of the pattern searched (参ったなぁ). If you need to get the address of the first character of the string found, use the `-b` flag:

```
$ sjisgrep -b 参ったなぁ Marie\ no\ Atelier\ -\ Salburg\ no\ Renkinjutsushi\ \(Japan\)\ \(Track\ 1\).bin
0x1BEA4 "（…夢かぁ。参ったなぁ）"
```

### Search for all instances of a pattern in a binary

Source: [Atelier Judie: The Alchemist of Gramnad](https://atelier.fandom.com/wiki/Atelier_Judie:_The_Alchemist_of_Gramnad).

![Judie no Atelier](static/judie_no_atelier.jpg)

Using the same command as the example above, it's possible to search for all instances of a string:

```
$ sjisgrep 調合 Judie\ no\ Atelier\ -\ Gramnad\ no\ Renkinjutsushi\ \(Japan\)\ \(v2.01\).iso
0x80D6B6 "液体を調合するときなどに使う。ガラスは溶けないので金属を溶かす薬品を扱うときにも便利。"
0x8117F6 "お婆さんのメモ帳によると「中和剤はニオイの少ない物ほど調合に役立つ」…らしい。"
0x81187E "お婆さんが残したメモ帳…なんだけど、２００年の時の流れで殆どが朽ちてなくなっちゃった。残ったのは本当に基本的な調合のレシピと、すっごく難しいレシピだけ。とほほ…。"
0x816168 "月は魔力を司るって言うけどね。ふふ…今ここで何か調合したら面白いことになるかしら…、ふふふ。"
0x82144E "これはさっきの調合失敗のせいね…。一体何が起こったんだろう…。"
0x8291AC "ユーディットは自分が調合に失敗して、２００年もの昔から飛ばされてきたことを話した…。"
0x82A5D0 "ユーディット。ちょっといい？…新しい薬を調合してみたんだけど、ちょっと味見してくれないかしら？"
```

There are more results, but I omitted them to save space.

## Building

Clone this repo and build it with cargo:

```
$ git clone 
$ cd sjisgrep
$ cargo build --release
$ ./target/release/sjisgrep --version
0.1.0
```

## Acknowledgements

- [Wikipedia's C implementation of the Boyer-Moore string-search algorithm](https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_string-search_algorithm#C_implementation).
- [Monkey-Moore](https://www.romhacking.net/utilities/513/) application. The source code was not disclosed though.

## License

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. See [LICENSE](LICENSE) for more details.
