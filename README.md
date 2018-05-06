# impact

An extraterrestrial WASM adventure.  Very much WIP.

It's incremental, but not realtime.

## Requirements

In development, I'm targeting the `wasm32-unknown-unknown` compiler target, which is currently nightly-only.  Stable should work if you use emscripten/asmjs.

I use a `package.json` for build scripts/JS-ecosystem deps.  My preferred tool is `yarn`, you can use `npm` as well.  Provided scripts:

* `clean` - Clean build artifacts.
* `build` - Build a production-ready bundle - useless for now, you can't serve this statically
* `start` - Start a development server and watch for changes.  Currently runs a release build only.

## Crates

* [stdweb](https://github.com/koute/stdweb) - DOM manipulation
* [yew](https://github.com/DenisKolodin/yew) - Client-side Rust->Wasm framework.  A little Elm-ish, a litte React-ish, a lot experimental

## Acknowledgements

Many thinks to [OvermindDL1](https://github.com/overminddl1)'s [blog series](http://blog.overminddl1.com/tags/overbots/).  A lot of this is similar to his OCaml structure where applicable, and going through that tutorial gave me the idea and structure to even attempt this.  The SCSS scripts and flexbox setup are his until I tackle my own.  While I decided not to go with realtime incremental for this project the basic engine is very similar to his Overbots structure at least at this point in development, but, you know, in `yew`.  His [`bucklescript-tea`](https://github.com/OvermindDL1/bucklescript-tea) is also an amazing project and you should check it out!

Also of course the [yew examples](https://github.com/DenisKolodin/yew/tree/master/examples) were invaluable in overcoming the humps.  I'm still not positive I'm getting the event pattern down proper but time will tell!