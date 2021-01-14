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

After you have started with [rust-agent-based-models], clone this repository. From a terminal, run:

```bash
npm install
npm start
```

This will invoke [`webpack-dev-server`] and automatically open the model in a web browser.

Once you have verified that the code builds, take some time to understand the similarities and differences between this project and [rust-agent-based-models]. Look for the `begin-similar-code` markers for blocks of code that both projects have in common, and then use this project for manually exploring parameters from your ABM.

You can also use `[diff]` to examine differences between the two programs, for example:

```bash
diff -u rust-agent-based-models/src/main.rs wasm-agent-based-models/src/lib.rs
```

To examine the similarities between the two programs, you can use [`fgrep`]:

```bash
fgrep -xf rust-agent-based-models/src/main.rs wasm-agent-based-models/src/lib.rs
```

When you are satisfied with your model implementation, use

```bash
npm run build
```

to build an optimized version of the model in the folder `target/app`. These are static files ready to be uploaded to a web server of your choice.

The live example at http://facorread.netlify.app/wasm-agent-based-models shows the epidemic model from the rust-agent-based-models project.

## Why make these two separate projects? / Do I need to copy code from one project into the other verbatim?

Sometimes you want to copy the full model from one project to another, and sometimes you want to explore changes to just one of the concepts or algorithms. These projects are complementary rather than redundant.

## Why Javascript? Typescript is safer.

I am eager to use Typescript as soon as possible. However, both webpack versions 5 and 4 bundle Wasm into the initial chunk when invoked from `app.ts`, which prevents Typescript from loading Wasm asynchronously. Wasm support is experimental in both Webpack and ts-loader. Contributing to said support seems like a better gift to the community than just hacking Typescript into this one project.

Let us stick to Javascript for now.

## Why is the animation so large? Can we adapt it to screen size instead?

The animation frames have the same size in both projects, 1920x1080. Websites have a logical representation that is usually smaller. I was aiming to make this a responsive project, able to adapt to any screen size. However, browsers do not work well when we resize the canvas element. I annotated file `app.scss` with specific examples. My recommendation is to manually adjust the browser zoom to fit your needs.

## How do we set the minimum, maximum, step size of the model parameters?

For the sliders that are accompanied by input boxes, such as the Simulation speed slider, please set the values in the `JsSliderValue` constructor in `app.js`. This is the preferred method because it matches the values between the two controls. Consider the values in `index.html` as placeholders for copying and pasting around. For the other controls, feel free to set the values in `index.html` or use Javascript.

## Why is the model so slow?

The model is in development mode by default, which enables debugging of the Rust and Javascript code; debugging is time-consuming. In file `webpack.config.js` you can change the mode to production, which speeds up the model considerably.

## Why is the website so unresponsive? It stutters and freezes a lot.
The website is running on a single computer thread, with simulation and rendering as the costliest operations. The thread also renders the user interface and handles interactions, as lower priority tasks. The only way to make the website more responsive is to offload the simulation and rendering to a web worker, which hinges on the standardization of [`OffscreenCanvas`]. I am happy to offload modeling work into a web worker as soon as [`OffscreenCanvas`] becomes available on Safari. In the mean time, you could try offloading modeling work into a web worker yourself. This would entail refactoring the model code, which would be quite time-consuming. Meanwhile, rendering work would stay on the main thread, which means stuttering and freezing would persist.

## Does this repository use `unsafe` code?

Not explicitly.

## Acknowledgements

This project uses the [wasm-pack-template] template by the [rust-wasm] team, the [create-wasm-app] template by Ashley Williams of the [rust-wasm] team, and [Material Components for the web].

[create-wasm-app]:https://github.com/rustwasm/create-wasm-app
[diff]:https://man7.org/linux/man-pages/man1/diff.1.html
[fgrep]:https://man7.org/linux/man-pages/man1/fgrep.1.html
[graphical user interfaces]:https://www.areweguiyet.com/
[Material Components for the web]:https://github.com/material-components/material-components-web
[`npm`]:https://www.npmjs.com/get-npm
[`OffscreenCanvas`]:https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas
[rust-agent-based-models]:https://github.com/facorread/rust-agent-based-models
[Rust and WebAssembly book]:https://rustwasm.github.io/docs/book/
[rust-wasm]:https://rustwasm.github.io/
[wasm-pack-template]:https://github.com/rustwasm/wasm-pack-template
[`webpack-dev-server`]:https://webpack.js.org/configuration/dev-server/
