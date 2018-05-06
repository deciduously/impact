# impact

An extraterrestrial WASM adventure.

## Requirements

In development, I'm targeting the `wasm32-unknown-unknown` compiler target, which is currently nightly-only.  Stable should work if you use emscripten/asmjs.

I use a `package.json` for build scripts/JS-ecosystem deps.  My preferred tool is `yarn`, you can use `npm` as well.  Provided scripts:

* `clean` - Clean build artifacts.
* `build` - Build a production-ready bundle - useless for now, you can't serve this statically
* `start` - Start a development server and watch for changes.  Currently runs a release build only.

## Libraries

* [stdweb](https://github.com/koute/stdweb) - DOM manipulation
* [yew](https://github.com/DenisKolodin/yew) - Client-side Rust->Wasm framework

## Acknowledgements

Many thinks to [OvermindDL1](https://github.com/overminddl1)'s [blog series](http://blog.overminddl1.com/tags/overbots/).