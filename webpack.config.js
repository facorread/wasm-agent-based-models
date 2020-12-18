const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "development",
  entry: {
    index: "/js/index.js"
  },
  experiments: {
    syncWebAssembly: true, // deprecated, see https://github.com/webpack/webpack/issues/11347
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    static: dist,
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        { from: path.resolve(__dirname, "static"), to: dist }
      ]
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ]
};
