# Polynomial SHA256 Example

This OpenVM example demonstrates polynomial evaluation with modular arithmetic and SHA256 hashing. The program evaluates a polynomial with given coefficients at a specific point, computing intermediate steps and their SHA256 hashes.

## Overview

The program:
1. Reads polynomial coefficients and an evaluation point from input
2. Evaluates the polynomial using modular arithmetic (modulus: 998244353)
3. Computes SHA256 hashes of intermediate evaluation steps
4. Reveals the final result and all intermediate values with their hashes

## Input Format

The program expects input in the following format:
- `n`: Number of polynomial coefficients (u32)
- `coeff_0, coeff_1, ..., coeff_{n-1}`: Polynomial coefficients (u32 values)
- `x`: Evaluation point (u32)

The polynomial is evaluated as: `coeff_{n-1} * x^{n-1} + ... + coeff_1 * x + coeff_0`

## Building and Running

### Prerequisites

Make sure you have the OpenVM toolchain installed and configured.

### Build the Program

```bash
cd polynomial_sha256
cargo openvm build
```

### Generate Inputs

To generate input files for different polynomial types, use the input generator:

```bash
# Generate inputs for constant, linear, and quadratic polynomials
cd ../utils
cargo run --bin generate_poly_inputs
cd ../polynomial_sha256
```

This will create three input files:
- `inputs/poly_constant.json` - Constant polynomial: f(x) = 5
- `inputs/poly_linear.json` - Linear polynomial: f(x) = 2x + 2  
- `inputs/poly_quadratic.json` - Quadratic polynomial: f(x) = x^2 + 2x + 1

### Run with OpenVM

Execute the program with one of the generated inputs:

```bash
# Run with constant polynomial
cargo openvm run --input ./inputs/poly_constant.json

# Run with linear polynomial  
cargo openvm run --input ./inputs/poly_linear.json

# Run with quadratic polynomial
cargo openvm run --input ./inputs/poly_quadratic.json
```

## Example Polynomials

### Constant Polynomial
- Coefficients: [5]
- Input: `n=1, coeff_0=5, x=3`
- Result: f(3) = 5

### Linear Polynomial  
- Coefficients: [2, 2]
- Input: `n=2, coeff_0=2, coeff_1=2, x=4`
- Result: f(4) = 2x4 + 2 = 10

### Quadratic Polynomial
- Coefficients: [1, 2, 1] 
- Input: `n=3, coeff_0=1, coeff_1=2, coeff_2=1, x=3`
- Result: f(3) = 1x3^2 + 2x3 + 1 = 16

## Custom Input Generation

You can generate custom polynomial inputs by modifying the input generator or creating the JSON format manually:

```rust
// Example: Create input for polynomial f(x) = x^2 + 2x + 1 at x=5
let n = 3u32;
let coefficients = vec![1u32, 2u32, 1u32]; // [constant, linear, quadratic]
let x = 5u32;

// Create separate input entries for each read() call
let mut input_entries = Vec::new();

// First entry: n
let n_hex = hex::encode(n.to_le_bytes());
input_entries.push(format!("\"0x01{}\"", n_hex));

// One entry for each coefficient
for coeff in coefficients {
    let coeff_hex = hex::encode(coeff.to_le_bytes());
    input_entries.push(format!("\"0x01{}\"", coeff_hex));
}

// Final entry: x
let x_hex = hex::encode(x.to_le_bytes());
input_entries.push(format!("\"0x01{}\"", x_hex));

// Create JSON
let json_content = format!("{{\n  \"input\": [\n    {}\n  ]\n}}", 
    input_entries.join(",\n    "));
std::fs::write("custom_input.json", json_content)?;
```

## Program Output

The program reveals the following as u32 values in the execution output:
- Final polynomial evaluation result (first value)
- Each intermediate evaluation step value  
- SHA256 hash of each intermediate step (first 4 bytes as u32)

Example output for constant polynomial f(x) = 5:
```
Execution output: [5, 0, 0, 0, 5, 0, 0, 0, 170, 231, 97, 55, ...]
                   ^final result    ^step value    ^SHA256 hash
```

All values are revealed as little-endian u32 and can be observed in the OpenVM execution trace.
