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

## Licence

Typst is Apache-2.0, and so are these changes. Each changed place is marked
in the source.
