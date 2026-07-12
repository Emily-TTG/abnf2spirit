# abnf2spirit

`abnf2spirit` is a CLI tool for converting ABNF grammars to
[`Boost::Spirit.X4`](https://github.com/boostorg/spirit_x4) grammar definitions.

Embedded in the repo is a MIME `Content-Type` parser employing the tool.

## Usage

`cargo run -- grammar.abnf template.inl.in output.inl`

`template.inl.in` should contain a `{{GRAMMAR}}` marker to be replaced by the
generated grammar definition.

`Spirit.X4` is still in early migration so there may be instabilities and
regressions until it reaches the Boost superproject.

## CMake Integration

This file is a part of the abnf2spirit
(https://github.com/Emily-TTG/abnf2spirit) project, which is used under
the terms of the MIT/X11 licence.
