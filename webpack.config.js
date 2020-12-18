const autoprefixer = require("autoprefixer");
const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const dist = path.resolve(__dirname, "dist");

module.exports = {
  entry: ["./app.scss", "./app.js"],
  output: {
    filename: "bundle.js",
  },
  module: {
    rules: [
      {
        test: /\.scss$/,
        use: [
          {
            loader: 'file-loader',
            options: {
              name: 'bundle.css',
            },
          },
          { loader: 'extract-loader' },
          { loader: 'css-loader' },
          {
            loader: 'postcss-loader',
            options: {
              postcssOptions: {
                plugins: [
                  autoprefixer()
                ]
              }
            }
          },
          {
            loader: 'sass-loader',
            options: {
              // Prefer Dart Sass
              implementation: require('sass'),

              // See https://github.com/webpack-contrib/sass-loader/issues/804
              webpackImporter: false,
              sassOptions: {
                includePaths: ['./node_modules'],
              },
            },
          }
        ],
      },
      {
        test: /\.js$/,
        loader: 'babel-loader',
        options: {
          presets: ['@babel/preset-env'],
        },
      }
    ],
  },
  devServer: {
    static: dist,
  },
  experiments: {
    syncWebAssembly: true, // deprecated, see https://github.com/webpack/webpack/issues/11347
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ]
};
