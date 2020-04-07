# rustymind

![CI](https://github.com/ondras/rustymind/workflows/CI/badge.svg)

This is a hobby project to learn Rust. It is an implementation of the *Mastermind* game, including a game-solving AI.

![screenshot](https://i.imgur.com/SXachN1.png)

## Running

```sh
$ git clone https://github.com/ondras/rustymind.git && cd rustymind
$ cargo run
```

With an explicit code length:

```sh
$ cargo run -- 5
```

Tests:

```sh
$ cargo test
```

## TODO

  - [X] GH Actions to lint, test, and release
  - [ ] the `--i-guess` option
