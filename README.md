# wasm-agent-based-models

Reliable and efficient agent-based models in Rust and WebAssembly

Copyright (C) 2020 Fabio Correa facorread@gmail.com

https://github.com/facorread/wasm-agent-based-models

https://gitlab.com/facorread/wasm-agent-based-models

## Introduction

Please check out the README for [rust-agent-based-models] first.

Agent-based models (ABM) are computer programs that define agents, virtual entities that imitate the decision-making processes and interactions of real people, animals, neurons, etc. [wasm-agent-based-models] is an interactive version of [rust-agent-based-models] that runs in a web browser. The researcher can copy code from one project into the other to explore model behavior. While [rust-agent-based-models] explores multiple, independent runs of the model under a wide range of parameter scenarios automatically, wasm-agent-based-models visualizes one run of the model while the researcher freely varies parameters, pauses the model, and explores alternative behavior using interactive controls.

## Why WebAssembly?

Rust offers a range of tools to develop [graphical user interfaces] (GUIs); in 2020, these projects were still experimental, but I needed stable tools for a modeling GUI. I decided to use WebAssembly because it has a longer track record and it is an open standard available on Windows, Linux, and Mac systems.

## Getting started

To get started with wasm-agent-based models, decent knowledge of html, css, and javascript is required. You should also read the [Rust and WebAssembly book], and follow the setup instructions therein. [`npm`] is required.

After you have started with [rust-agent-based-models], [Clone this repository] (`https://github.com/facorread/wasm-agent-based-models.git`) and use your favorite terminal to run `wasm-pack build --target web`. Follow the instructions on screen.

Once you have verified that the code builds, take some time to understand the similarities and differences between this project and [rust-agent-based-models]. Look for the `begin-similar-code` markers for blocks of code that both projects have in common, and then use this project for manually exploring parameters from your ABM.

You can also use `[diff]` to examine differences between the two programs, for example:

```bash
diff -u rust-agent-based-models/src/main.rs wasm-agent-based-models/src/lib.rs
```

To examine the similarities between the two programs, you can use `[fgrep]`:

```bash
fgrep -xf rust-agent-based-models/src/main.rs wasm-agent-based-models/src/lib.rs
```

## Why make these two separate projects? / Do I need to copy code from one project into the other verbatim?

Sometimes you want to copy the full model from one project to another, and sometimes you want to explore changes to just one of the concepts or algorithms. These projects are complementary rather than redundant.

## Why Javascript? Typescript is safer.

I am eager to use Typescript as soon as possible. However, both webpack versions 5 and 4 bundle Wasm into the initial chunk when invoked from `app.ts`, which prevents Typescript from loading Wasm asynchronously. Wasm support is experimental in both Webpack and ts-loader. Contributing to said support seems like a better gift to the community than just hacking Typescript into this one project.

Let us stick to Javascript for now.

## Does this repository use `unsafe` code?

Not explicitly.

## Acknowledgements

This project was created using the [wasm-pack-template] template by the [rust-wasm] team.

The `www/` directory comes from the [create-wasm-app] template by Ashley Williams of the [rust-wasm] team.

[create-wasm-app]:https://github.com/rustwasm/create-wasm-app
[diff]:https://man7.org/linux/man-pages/man1/diff.1.html
[fgrep]:https://man7.org/linux/man-pages/man1/fgrep.1.html
[graphical user interfaces]:https://www.areweguiyet.com/
[`npm`]:https://www.npmjs.com/get-npm
[rust-agent-based-models]:https://github.com/facorread/rust-agent-based-models
[Rust and WebAssembly book]:https://rustwasm.github.io/docs/book/
[rust-wasm]:https://rustwasm.github.io/
[wasm-pack-template]:https://github.com/rustwasm/wasm-pack-template
