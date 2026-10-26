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

## Licence

Typst is Apache-2.0, and so are these changes. Each changed place is marked
in the source.
