# Developing

Install system packages

```bash
sudo apt install build-essential clang libwebkit2gtk-4.1-dev curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

Install rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install wasm32 target

```bash
rustup target add wasm32-unknown-unknown
```

Optionally install rust-analyzer

```bash
rustup component add rust-src
rustup component add rust-analyzer
# Next configure your editor to use it, see instructions on the web
```

Install dioxus


```bash
cargo update dioxus-cli
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to http://localhost:8080


## Dioxus

This is written in [Dioxus](https://dioxuslabs.com/) which is a full-stack cross-platform
development framework based in rust that is similar to flutter and NextJS.
