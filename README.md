# impact
An incremental game skeleton. Very much WIP.

It's incremental, but not realtime.  For now, a second passes each time an Action happens, which is not ideal...bulk actions will take way too long.  Eventually I will fine tune this so that each action has a duration, meaning adding messages to the console won't clog it up.

![screenhot](https://i.imgur.com/siTfdFc.png).

Play the first thrilling 30 seconds [here](http://deciduously.com/impact).

## Requirements

* [Rust](https://www.rust-lang.org/en-US/) 2018 with the `wasm32-unknown-unknown` target.
* [cargo-web](https://github.com/koute/cargo-web)
* [NPM/Node](https://nodejs.org/en/download/)

You can use stable rust and Emscripten/asmjs but do you really *want* to?

## Usage

* `yarn clean` - Clean build artifacts.
* `yarn dev` - Start a development server on `localhost:8000` and watch for changes to either Rust or SCSS.
* `yarn prod` - Build a production bundle at `release/` and serve on `localhost:8080`.

Note - the `wasm32-unknown-unknown` target does not currently support debug builds, so the dev server runs a release build of the rust code.  For now, the only difference with with the production build is the JS minifier (negligible gain) and lack of hot reloading.

## Crates

* [stdweb](https://github.com/koute/stdweb) - DOM manipulation
* [yew](https://github.com/DenisKolodin/yew) - Client-side Rust->Wasm framework.  A little Elm-ish, a litte React-ish, a lot experimental

## Acknowledgements

Many thinks to [OvermindDL1](https://github.com/overminddl1)'s [blog series](http://blog.overminddl1.com/tags/overbots/).  A lot of this is similar to his OCaml structure where applicable, and going through that tutorial gave me the idea and structure to even attempt this.  The SCSS scripts and flexbox setup are his until I tackle my own.  While I decided not to go with realtime incremental for this project the basic engine is very similar to his Overbots structure at least at this point in development, but, you know, in `yew`.  His [`bucklescript-tea`](https://github.com/OvermindDL1/bucklescript-tea) is also an amazing project and you should check it out!

Also of course the [yew examples](https://github.com/DenisKolodin/yew/tree/master/examples) were invaluable in overcoming the humps.  I'm still not positive I'm getting the event pattern down proper but time will tell!
