# MiniJinja Playground

Prototype application for MiniJinja for the purpose of testing and demonstration on the web.

## Building

Ensure you have [wasm-pack](https://github.com/rustwasm/wasm-pack).

Then running:

```sh
wasm-pack build --target no-modules --release
```

Will build the project for you.

You will need to ensure you are running a local web server to serve the files.