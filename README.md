# OpenVM Examples

This repo demonstrates three simple OpenVM guest program examples along with some handy utilities for working with OpenVM programs.

It contains four crates:

## Guest Crates
- [`openvm-example-fibonacci`](./fibonacci/): an example Fibonacci program. See the [book](https://docs.openvm.dev/book/writing-apps/overview) for more details.
- [`diffie-hellman-example`](./diffie_hellman/): an example Diffie Hellman Key Exchange program. This program takes in two inputs as Alice's and Bob's private keys and generates a shared key hashed with SHA-256.
- [`polynomial_sha256`](./polynomial_sha256/): an example demonstrating polynomial evaluation with modular arithmetic and SHA256 hashing, showcasing multiple VM extensions.

## Utility Crates
- [`openvm-example-utils`](./utils/): a set utility functions to generate OpenVM program inputs in the right format. Since the Fibonacci program takes a `u64` as input, you can use the [`generate_input_from_u64`](./utils/examples/gen_input.rs#L13) function to generate the input.
