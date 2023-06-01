# denominator template

Pure rust SPA relying on fast, minimal signal lib +> [rust-dominator](https://github.com/Pauan/rust-dominator)

Based on [dominator-tailwind-boilerplate](https://github.com/dakom/dominator-tailwind-boilerplate), with:

* Simplification (less biolerplate)
* Enhancement (with dark mod)
* Bug fix (regarding routing)

To know more about the core of [rust-dominator](https://github.com/Pauan/rust-dominator) read this:

[tutorial](https://docs.rs/futures-signals/0.3.32/futures_signals/tutorial/index.html)

You will need to install trunk:

```sh
cargo install --locked trunk
```

## Features

- tailwindcss
- dark mode
- routing
- layout

## Instruction

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

Then serve content in `./dist` with an http server
