const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8000
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "./static/bk-over-ssh.[hash].hashed.js",
      webassemblyModuleFilename: "./static/bk-over-ssh.[hash].hashed.wasm"
    },
    plugins: [
      new CleanWebpackPlugin(),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      }),
      new HtmlWebpackPlugin({
        template: "static/index.html"
      }),
      new CopyWebpackPlugin([
        { from: './static', to: distPath },
        { from: 'purecss/build/**/*min.css', to: distPath + '/static/purecss', context: './node_modules/', flatten: true }
      ])
    ],
    watch: argv.mode !== 'production'
  };
};
