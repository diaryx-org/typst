# diaryx-org's patches to Typst

This fork exists so that [twig-export](https://github.com/diaryx-org/twig-export)
can lay out Typst content it builds in Rust from a Markdown tree, with a Typst
template for the procedural part of a page. The argument is twig-export's
`docs/proposals/typst-is-forked-and-kept-under-markdown.md`.

The branch `twig` is an upstream release tag plus the patches below, and
commits that change only this file. At each Typst release it is rebased onto the new tag. Each patch
goes when upstream has an equivalent.

Base: `v0.15.1` (`9dfd3a0`).

## `Span::from_range` is public

`crates/typst-syntax/src/span.rs`. Was `pub(crate)`.

Content built outside Typst markup has no source for Typst's numbered spans.
A raw-range span lets each element carry a place in the Markdown it came from,
so a laid-out glyph (and a diagnostic) leads back to that byte. Without this,
the span's bits have to be packed by hand through `Span::from_raw`, which
breaks quietly if the layout changes.

Upstream: not yet requested.

## `typst::compile_with`

`crates/typst/src/lib.rs`. New.

`compile` lays out the main file's content. `compile_with` evaluates the main
file the same way, then lays out what a callback builds from the resulting
module, once per layout iteration, with that iteration's engine. A caller can
call a template function from the main file with content built in Rust.

The content must not go in through the `World` or the `Library`: every memoized
layout depends on the library's hash, so a body passed as a global makes every
change a cold compile. Without this function a caller copies `compile_impl`'s
layout loop.

`compile` and `trace` behave as before: `compile_impl` takes the body as a
callback, and they pass one that returns the module's content.

Upstream: not yet requested.

## `par.overflow-wrap`

`crates/typst-library/src/model/par.rs` (the property and `OverflowWrap`),
`crates/typst-layout/src/inline/mod.rs` (read into the line breaker's
configuration) and `crates/typst-layout/src/inline/linebreak.rs` (the
breaking). Tests in `tests/suite/layout/inline/linebreak.typ`, the
`linebreak-overflow-wrap-*` ones.

Typst breaks a line only where Unicode's line breaking rules or hyphenation
allow, and a run with no such place that is wider than the line overflows it:
a long URL, an identifier, a string of punctuation. A typesetter has always
left that for the author to fix. An editor cannot: the text is the person's,
shown at the width of their window. `#set par(overflow-wrap: "break-word")`
breaks such a run between any two graphemes, but only where it cannot fit
otherwise, as CSS's `overflow-wrap: break-word` has it.

`"normal"`, the default, is Typst as it was. With `"break-word"`, a paragraph
that comes out with no overfull line is broken once, as before. One that has
an overfull line is broken again, with each run between two break
opportunities that is wider than a line of its own given an emergency
breakpoint between each two of its graphemes. The simple breaker ends a line
at its last ordinary breakpoint that fits rather than at an emergency one, so
a run starts a line of its own first; the optimized breaker costs an
emergency break far above a hyphenation and far below an overfull line, so it
takes one only where nothing else fits.

Upstream: not yet requested. A good candidate: upstream has no way to keep a
long run on the page.

## Feature gates

`Cargo.toml` (the workspace's `typst-library`, `typst-pdf`, `typst-svg`,
`typst-html` and `krilla`) and the manifests of `typst`, `typst-library`,
`typst-pdf`, `typst-svg`, `typst-html` and `typst-render`, with the members
that build everything (`typst-cli`, `typst-bundle`, `typst-ide`, `docs`,
`tests`, `tests/fuzz`) naming `pdf-images`. In the source: `typst-library`'s
`foundations/plugin.rs`, `loading/mod.rs`, `model/bibliography.rs`,
`visualize/image/mod.rs` and the new `visualize/image/pdf_off.rs`;
`typst-pdf`'s `convert.rs` and `image.rs`; `typst-svg`'s `image.rs`.

Upstream Typst has no features to leave a part of the standard library out,
and each part below brings crates of its own: Hayagriva alone is 27, with
ICU's collator and its data. twig-export uses none of them, and each one is
build time and binary size in every app that sets a page with it. Each part is
now a feature of `typst-library`, forwarded by the `typst` facade:

| Feature | What it is | What it brings |
|---|---|---|
| `bibliography` | Reading and rendering works for `bibliography` and `cite` | hayagriva, citationberg, biblatex, icu_collator, … (turns on `yaml`) |
| `plugin` | `plugin`, WebAssembly plugins | wasmi |
| `csv`, `yaml`, `cbor` | Those data loaders | csv, serde_yaml, ciborium |
| `pdf-images` | A PDF page as an `image` | hayro-syntax; in `typst-pdf` krilla's `pdf`, in `typst-svg` (and so `typst-html`) hayro and hayro-svg |

All are on by default, so a build that names no features is upstream Typst.
For a downstream to turn them off, the workspace's `typst-library`,
`typst-pdf`, `typst-svg` and `typst-html` dependencies say
`default-features = false`:
otherwise one member's dependency (`typst-layout`'s, say) would turn back on,
through Cargo's feature unification, what the downstream turned off. The
`typst` facade's defaults turn the library's features on, and `typst-html`'s
`pdf-images` (it draws frames with `typst-svg`); the members that build
everything name `pdf-images` for `typst-pdf`, `typst-svg` and `typst-html`.
With the patch, the third-party crates of `typst-cli`'s, the `typst` facade's
and the whole workspace's dependency graphs are unchanged, crate for crate and
feature for feature. `typst-render` always draws PDF images.

Without a feature, what a document can name stays as it was wherever that is
cheap, so that a template or package that only mentions a part still works.
Hydra, for one, lists `bibliography` and `cite` among the elements it queries.

- `bibliography`: both elements exist, cast, and take set and show rules.
  A local `mod hayagriva` stands in for the few Hayagriva types they name: a
  style is named but never read, and any name is accepted. Calling
  `bibliography` fails ("bibliographies are not supported by this build of
  Typst"), so a document has none, and a citation fails as it does in a
  document without one.
- `plugin`: `plugin` exists and fails. `Plugin` is an uninhabited stand-in, so
  `PluginFunc` and `func.rs` are unchanged.
- `csv`, `yaml`, `cbor`: the functions are not defined.
- `pdf-images`: `PdfImage` is an uninhabited stand-in, `pdf_off.rs`, so
  `ImageKind::Pdf` stays; loading a PDF as an image fails. `typst-pdf` without
  the feature fails on a PDF image, and `typst-svg` draws it as nothing; either
  arises only when another crate turns the library's feature on.

Kept on purpose, with no gate: `toml`, `xml` and `json`, whose crates the
library needs anyway (`typst-syntax` reads `typst.toml`; `roxmltree` reads SVG
glyphs); `lorem` (lipsum), which twig-export's examples use; syntect and
two-face, for highlighting `raw` blocks.

Upstream: not requested.

## Licence

Typst is Apache-2.0, and so are these changes. Each changed place is marked
in the source.
