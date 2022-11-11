//@ts-check

"use strict"

const path = require("path")

//@ts-check
/** @typedef {import('webpack').Configuration} WebpackConfig **/

/** @type WebpackConfig */
const extensionConfig = {
  target: "node",

  // Leaves the source code as close as possible to the original (when packaging
  // we set this to `production` instead of `none`).
  mode: "none",

  entry: "./src/extension.ts",

  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "extension.js",
    libraryTarget: "commonjs2",
  },

  externals: {
    vscode: "commonjs vscode",
  },

  resolve: {
    extensions: [".ts", ".js"],
  },

  module: {
    rules: [
      {
        test: /\.ts$/,
        exclude: /node_modules/,
        use: [
          {
            loader: "ts-loader",
          },
        ],
      },
    ],
  },

  devtool: "nosources-source-map",

  infrastructureLogging: {
    level: "log",
  },
}

module.exports = [extensionConfig]
