# MiniJinja Playground

MiniJinja Playground that allows you to experiment with MiniJinja, completely in your browser!

This uses the magic of [WebAssembly](https://webassembly.org/) to run MiniJinja in your browser.

## Building
### Requirements
1. [wasm-pack](https://github.com/rustwasm/wasm-pack) installed to path `cargo install wasm-pack`
2. NodeJS with npm (Tested with NodeJS 18.13.0 LTS and 19.4.0)

### Installing
1. Install the dependencies using npm:
```sh
npm install
```

### Running
2. To run the dev server:
```sh
npm run liveserver
```

This will start listening for changes, then on change will run:
- `wasm-pack` (for building WebAssembly)
- `tailwind` (for CSS)

It also runs `live-server`, which listens on `127.0.0.1` on port `8080`.

You should then be able to access the local build on http://127.0.0.1:8080 .

### Deploying
@todo :)

## What this is
This is a project to demonstrate how to use MiniJinja, and allow replicating issues for the issue tracker.

Right now it's pretty basic!

## Roadmap
### Short term
- [ ] Add debug & production level builds, right now, it's fully `debug`.
- [ ] Add the ability to share a link via a URL.
- [ ] Validate JSON object for the template variables.
- [ ] Validate the template is valid MiniJinja, show errors in a nicer way.
- [ ] Add Monaco highlighting for the template & objects.
- [ ] Build a "main" branch version and versioned releases, so testing stable and earlier versions of MiniJinja is possible.
- [ ] Add WASM tests
- [ ] Move the MiniJinja template into a better spot
- [ ] Improve UX & Theme (I'm a developer, so less developer design :-))

### Medium term
- [ ] Deploy automatically via CI, once the above is semi-stable.
- [ ] Add integration tests.
- [ ] Add ability for users to toggle MiniJinja feature flags.
- [ ] Use web workers to parse MiniJinja templates, so the UI doesn't freeze.

## Architecture
This is a Rust project, built using the following:

### wasm-pack
[WASM Pack](https://github.com/rustwasm/wasm-pack) is used for building the Rust project into web assembly.
You may need to install this via `cargo install wasm-pack`,  and if you have issues you might need
[this branch](https://github.com/rustwasm/wasm-pack/pull/1188).

### wasm-bindgen
[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) is used as the glue that ties everything together.

### web-sys
[web-sys](https://github.com/rustwasm/wasm-bindgen/tree/main/crates/web-sys) is used extensively to interact with the DOM.

### Tailwind
[Tailwind](https://tailwindcss.com/) is used to style the website and components. It's pretty nice.

### MiniJinja
Try to use as much MiniJinja to ["dog food"](https://en.wikipedia.org/wiki/Eating_your_own_dog_food) as much as possible!

## LICENSE
This project is licensed under the Apache 2.0 license. See [LICENSE](LICENSE) for more information.