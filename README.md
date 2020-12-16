# Advent of Code 2020

Hi, there! I've been wanting to learn [Rust](https://www.rust-lang.org/) for a while and this year's Advent of Code finally gave me a good reason to do so.

In this repository you will find my solutions to the challenges.
Please note that this is my first ever time programming in Rust, so I'm sure that if you are a seasoned Rustacean you will find many issues or non-idiomatic code if you look close enough.

I tried to include tests with most of my solutions such that I could test my programs before submitting an answer.

## Usage

If you want to run any of the examples, then you should install Rust through [rustup](https://www.rust-lang.org/tools/install).
After you've installed Rust you should be able to run the binaries using [cargo](https://doc.rust-lang.org/cargo/guide/index.html):

```bash
cat data/day-1/input.txt | cargo run --bin day_1_1
```

Every challenge comes with its own binary, the binaries can be identified by the "day" and the "part" of the challenge for that particular day.

You can also use cargo to run tests, in order to do this execute:

```bash
cargo test
```
