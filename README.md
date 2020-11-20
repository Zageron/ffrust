# FFRUST

- [FFRUST](#ffrust)
  - [Setup](#setup)
    - [Windows Subsystem for Linux](#windows-subsystem-for-linux)
    - [Rust](#rust)
    - [wasm-pack](#wasm-pack)
    - [Cargo Generate](#cargo-generate)
    - [Node](#node)
      - [Install nvm](#install-nvm)
      - [Install node](#install-node)
      - [Install ncu](#install-ncu)
    - [NPM](#npm)
      - [Check for existing npm](#check-for-existing-npm)
      - [Install npm](#install-npm)
      - [Update npm](#update-npm)
  - [Develop](#develop)
    - [Build and Run](#build-and-run)
    - [Updating Cargo.toml](#updating-cargotoml)
  - [Reference](#reference)
    - [Rust wasm-bindgen](#rust-wasm-bindgen)

## Setup

### Windows Subsystem for Linux

Do the following, in order.

### Rust

If you’re a Windows Subsystem for Linux user run the following in your terminal, then follow the on-screen instructions to install Rust.

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### wasm-pack

If you're a Windows Subsystem for Linux user run the following in your terminal, then follow the onscreen instructions to install wasm-pack.

`curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`

### Cargo Generate

`cargo install cargo-generate`

### Node

Follow these instructions in order.

#### Install nvm

`wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.37.0/install.sh | bash`

#### Install node

`nvm install node`

#### Install ncu

ncu is used to keep your dependencies up to date.

`npm i -g npm-check-updates`

### NPM

npm should be installed.

#### Check for existing npm

`npm list -g --depth 0`

#### Install npm

`npm install npm@latest -g`

#### Update npm

The previous install command probably didn't install the latest npm.

`ncu -g`

## Develop

### Build and Run

```zsh
cargo build
wasm-pack build
cd www
npm install
npm run start
```

### Updating Cargo.toml

If you make any changes to Cargo.toml, run `cargo build`.

## Reference

### Rust wasm-bindgen

[https://rustwasm.github.io/wasm-bindgen/api/web_sys/index.html](https://rustwasm.github.io/wasm-bindgen/api/web_sys/index.html)
