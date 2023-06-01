# denominator template

Pure rust SPA relying on fast, minimal signal lib +> [rust-dominator](https://github.com/Pauan/rust-dominator)

Based on [credit template](https://github.com/dakom/dominator-tailwind-boilerplate), with:

* simplification (less biolerplate)
* enhanced version (with dark mod)
* with bug fix (regarding routing)

To know more about the core of [rust-dominator](https://github.com/Pauan/rust-dominator) read this:

[tutorial](https://docs.rs/futures-signals/0.3.32/futures_signals/tutorial/index.html)

You will need to install trunk:

```sh
cargo install --locked trun
```

## Features

- tailwindcss
- dark mode
- routing

## Instruction

### dev mode

```sh
pn i
pn dev
```

### release mode

```sh
pn dist
```

Then serve content in `./dist` with an http server

## Additional ressources

- [rust-dominator](https://github.com/Pauan/rust-dominator)
- [dominator-tailwind-boilerplate(https://github.com/dakom/dominator-tailwind-boilerplate)
