# wasm-agent-based-models

Reliable and efficient agent-based models in Rust and WebAssembly

Copyright (C) 2020 Fabio Correa facorread@gmail.com

https://github.com/facorread/wasm-agent-based-models
https://gitlab.com/facorread/wasm-agent-based-models

## Introduction

Please check out the README for [rust-agent-based-models first].

Agent-based models (ABM) are computer programs that define agents, virtual entities that imitate the decision-making processes and interactions of real people, animals, neurons, etc. [wasm-agent-based-models] is an interactive version of [rust-agent-based-models] that runs in a web browser. The researcher can copy code from one project into the other to explore model behavior. While [rust-agent-based-models] explores multiple, independent runs of the model under a wide range of parameter scenarios automatically, wasm-agent-based-models visualizes one run of the model while the researcher freely varies parameters, pauses the model, and explores alternative behavior using interactive controls.

## Why WebAssembly?

Rust offers a range of tools to develop [graphical user interfaces] (GUIs); in 2020, these projects were still experimental, but I needed stable tools for a modeling GUI. I decided to use WebAssembly because it has a longer track record and it is an open standard available on Windows, Linux, and Mac systems.

## Getting started

To get started with wasm-agent-based models, decent knowledge of html, css, and javascript is required. You should also read the [Rust and WebAssembly book], and follow the setup instructions therein. Installing `npm` is optional.

After you have started with [rust-agent-based-models], [Clone this repository] (`https://github.com/facorread/wasm-agent-based-models.git`) and use your favorite terminal to run `wasm-pack build --target web`. Follow the instructions on screen.

Once you have verified that the code builds, take some time to understand the similarities and differences between this project and [rust-agent-based-models]. This allows you to see the code that needs to be copied from one project into the other, and decide what parameters you want to explore as you work on designing your own ABM.

To examine the differences between the two programs, you can use `[diff]`, for example:

```bash
diff -u rust-agent-based-models/src/main.rs wasm-agent-based-models/src/lib.rs
```

To examine the similarities between the two programs, you can use `[fgrep]`:

```bash
fgrep -xf rust-agent-based-models/src/main.rs wasm-agent-based-models/src/lib.rs
```

## Do I need to copy code from one project into the other verbatim?

Probably not. Sometimes you want to copy the full model from one project to another, and sometimes you want to explore just one of the concepts or change one of the algorithms.

## Does this repository use `unsafe` code?

Not explicitly.

## Acknowledgement

This project was created using the [wasm-pack-template] template by the [rust-wasm] team.

[diff]:https://man7.org/linux/man-pages/man1/diff.1.html
[fgrep]:https://man7.org/linux/man-pages/man1/fgrep.1.html
[graphical user interfaces]:https://www.areweguiyet.com/
[rust-agent-based-models]:https://github.com/facorread/rust-agent-based-models
[Rust and WebAssembly book]:https://rustwasm.github.io/docs/book/
[rust-wasm]:https://rustwasm.github.io/
[wasm-pack-template]:https://github.com/rustwasm/wasm-pack-template