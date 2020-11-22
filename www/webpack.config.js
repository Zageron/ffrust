const CopyPlugin = require("copy-webpack-plugin");

const path = require('path');

module.exports = {
  entry: "./src/bootstrap.js",
  devtool: 'inline-source-map',
  output: {
    path: path.resolve(__dirname, "./dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  experiments: {
    syncWebAssembly: true
  },
  resolve: {
    extensions: [ '.ts', '.js' ],
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  plugins: [
    new CopyPlugin(
      {
        patterns: [
          {
            from: 'src/index.html',
            to: 'index.html',
            force: true,
          },
          {
            from: 'src/favicon.ico',
            to: 'favicon.ico',
            force: true,
          },
          {
            from: 'src/style.css',
            to: 'style.css',
            force: true,
          }
        ]
      })
  ],
};
