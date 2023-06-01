# denominator template

Pure rust SPA relying on fast, minimal signal lib +> rust-denominator
It's a simplified and enhanced version with bug fix of a more exhaustive template see ressources at the bottom of the doc.

To know more aboput teh core lib read this:

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

- [core lib](https://github.com/Pauan/rust-dominator)
- [credit template](https://github.com/dakom/dominator-tailwind-boilerplate)
