# Third-Party Notices

Poodle includes the following third-party assets. These assets remain under
their upstream licenses; Poodle's MIT license does not replace those terms.

## Lucide Icons

Lucide 1.31.0 supplies both the scoped icon data in
`packages/core/src/icons/icons/` and the SVG assets in
`packages/render/assets/icons/`. Both outputs are generated from one
canonical Poodle manifest. [Lucide](https://github.com/lucide-icons/lucide) is
licensed under the ISC License and includes icons derived from Feather under
the MIT License.

- Published core-package notice: [`packages/core/THIRD_PARTY_NOTICES.md`](packages/core/THIRD_PARTY_NOTICES.md)
- Rust asset license: [`packages/render/assets/icons/LICENSE.txt`](packages/render/assets/icons/LICENSE.txt)

## Inter

The Inter 4.001 font files in `packages/gpui/preview/assets/fonts/` come from
[Inter](https://github.com/rsms/inter) and remain licensed under the SIL Open
Font License, Version 1.1.

- Font license: [`packages/gpui/preview/assets/fonts/LICENSE.txt`](packages/gpui/preview/assets/fonts/LICENSE.txt)

## CodeMirror 6

The web `CodeEditor` (g18.002) runs on CodeMirror 6 behind the dedicated
`./editor` entries of `@inflatable-cookie/poodle-svelte` and
`@inflatable-cookie/poodle-react`. All packages are pinned exactly in both
shell manifests and remain under the MIT License:
[@codemirror/state 6.7.4](https://codemirror.net/),
[@codemirror/view 6.43.11](https://codemirror.net/),
[@codemirror/commands 6.11.0](https://codemirror.net/),
[@codemirror/search 6.7.2](https://codemirror.net/),
[@codemirror/language 6.12.4](https://codemirror.net/),
[@codemirror/lang-javascript 6.2.5](https://codemirror.net/),
[@codemirror/lang-json 6.0.2](https://codemirror.net/),
[@codemirror/lang-html 6.4.12](https://codemirror.net/),
[@codemirror/lang-css 6.3.1](https://codemirror.net/),
[@codemirror/lang-markdown 6.5.2](https://codemirror.net/),
[@codemirror/lang-rust 6.0.2](https://codemirror.net/),
[@codemirror/lang-yaml 6.1.3](https://codemirror.net/), and
[@codemirror/legacy-modes 6.5.4](https://codemirror.net/) (TOML and shell
stream parsers only).
