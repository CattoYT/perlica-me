# [Perlica.me](https://perlica.me)

Currently my personal website, designed around Perlica from Arknights: Endfield
Written entirely* in Rust because I was too stubborn to use anything else (big mistake)

Made with tears and suffering:tm:

### Compile from source

This requires [Trunk](https://github.com/trunk-rs/trunk) to be installed

1. ```git pull https://github.com/cattoyt/perlica-me```
2. ```cd perlica-me```
3. ```rustup toolchain install wasm32-unknown-unknown``` <- Skip if you already have the toolchain installed
4. ```trunk build```

### Stack
- Rust
- Leptos
- Cloudflare
- GH actions (builds and uploads to cf)
-
