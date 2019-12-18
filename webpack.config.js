const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");

const manually_version = "20191218";

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
      filename: "./static/hashed/bk-over-ssh.[hash].js",
      webassemblyModuleFilename: "./static/hashed/bk-over-ssh.[hash].wasm"
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
        { from: './static/static', to: distPath + '/static/hashed/' + manually_version },
        { from: 'purecss/build/**/*min.css', to: distPath + '/static/hashed/' + manually_version + '/purecss', context: './node_modules/', flatten: true }
      ])
    ],
    watch: argv.mode !== 'production'
  };
};
