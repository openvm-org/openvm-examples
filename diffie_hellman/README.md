## How to run this example

1. The `diffie_hellman/src/main.rs` program takes in two inputs - Alice's private key and Bob's private key. 
2. To pass in the input, first generate an `input.json` file by running `generate_input_from_u64()` in `utils/examples/gen_input.rs`
3. The `input.json` is of the form `{"input": ["hex_1", "hex_2"]}`. Example `input.json` can be seen at `diffie_hellman/input.json`
4. To obtain `hex_1`, first change the `input` variable inside the `generate_input_from_u64()` function to Alice's private key
5. Then do `cargo run` from inside the `utils` directory
6. Then `hex_1` will be printed out to `Stdout`
7. Repeat steps 4-6 to obtain `hex_2` with changing the `input` variable to Bob's private key this time
8. Then place in `hex_1` and `hex_2` into the `diffie_hellman/input.json` following the format in step 3
9. Then from inside the `diffie_hellman` directory, run `cargo openvm run --input input.json` to run the program
