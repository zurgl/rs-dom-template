# rust-dominator template

Pure rust SPA template relying on a fast and minimal signal lib:
* [rust-dominator](https://github.com/Pauan/rust-dominator)

This repo a minimal reproduction of [dominator-tailwind-boilerplate](https://github.com/dakom/dominator-tailwind-boilerplate), with:

* Minimal approch (less biolerplate and example)
* Add-on (with dark mode)
* Improvement (regarding routing)

To know more about the core of [rust-dominator](https://github.com/Pauan/rust-dominator) read this [tutorial](https://docs.rs/futures-signals/0.3.32/futures_signals/tutorial/index.html)

## Features

- tailwindcss
- dark mode
- routing
- layout

## Instruction

You will need to install trunk:

```sh
cargo install --locked trunk
```

Easy way to install pnpm if you have a working nodejs

```sh
corepack enable
corepack prepare pnpm@8.5.1 --activate
```

### dev mode

```sh
pnpm i
pnpm dev
```

### release mode

```sh
pnpm dist
```

Then serve the content in `./dist` with a http server
