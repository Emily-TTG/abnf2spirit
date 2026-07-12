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

You can consume the generator directly by including the `Generate.cmake` file
which exposes the generate tool as `abnf2spirit_generate_grammar`.

If you want to use the MIME parser lib, you can `add_subdirectory` the project
root which will provide the `abnf2spirit::mime` target. You will need to have
the `Boost::spirit_x4` and `Boost::fusion` targets in the parent directory's
scope.

This file is a part of the abnf2spirit
(https://github.com/Emily-TTG/abnf2spirit) project, which is used under
the terms of the MIT/X11 licence.
