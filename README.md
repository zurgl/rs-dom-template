# rust-dominator template

Pure rust SPA template relying on a fast and minimal signal lib:

* [rust-dominator](https://github.com/Pauan/rust-dominator)

This repo is a minimal reproduction of [dominator-tailwind-boilerplate](https://github.com/dakom/dominator-tailwind-boilerplate), with:

* Minimal approch (less biolerplate and example)
* Add-on (with dark mode)
* Improvement (regarding routing)

To know more about the core of [rust-dominator](https://github.com/Pauan/rust-dominator) read this [tutorial](https://docs.rs/futures-signals/0.3.32/futures_signals/tutorial/index.html)

## Features

* tailwindcss
* dark mode
* routing
* layout

## Instruction

You will need to install trunk:

```sh
cargo install --locked trunk
```

Easy way to install pnpm if you have a working nodejs

```sh
corepack enable pnpm
corepack use pnpm@latest
```

### dev mode

```sh
pnpm i
pnpm dev
```

### release mode

```sh
pnpm build
```

Then serve the content in `./dist` with a http server


### Maintenance mode

#### node maintenance

Upgrade pnpm version
```bash
pnpm upgrade
```

Check oudated `node_modules`
```bash
pnpm outdated
```

Updgrade `node module`
```bash
pnpm up --latest
```

#### rust maintenance

Check upgrade toolchain version
```bash
rustup upgrade
```

Set toolchain version
```bash
rustup set <toolchain>
```

