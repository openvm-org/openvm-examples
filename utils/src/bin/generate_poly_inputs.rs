use openvm_example_utils::Error;

fn create_input_file(n: u32, coefficients: Vec<u32>, x: u32, filename: &str) -> Result<(), Error> {
    let mut input_entries = Vec::new();

    // First entry: n
    let n_hex = hex::encode(n.to_le_bytes());
    input_entries.push(format!("\"0x01{}\"", n_hex));

    for coeff in coefficients {
        let coeff_hex = hex::encode(coeff.to_le_bytes());
        input_entries.push(format!("\"0x01{}\"", coeff_hex));
    }

    // Final entry: x
    let x_hex = hex::encode(x.to_le_bytes());
    input_entries.push(format!("\"0x01{}\"", x_hex));

    let json_content = format!(
        "{{\n  \"input\": [\n    {}\n  ]\n}}",
        input_entries.join(",\n    ")
    );

    std::fs::write(filename, json_content).map_err(|e| Error::Io(e))?;
    Ok(())
}

/// Generate input for a constant polynomial: f(x) = c
fn generate_constant_polynomial() -> Result<(), Error> {
    println!("Generating constant polynomial: f(x) = 5");

    let n = 1u32;
    let coefficients = vec![5u32]; // f(x) = 5
    let x = 3u32;

    create_input_file(
        n,
        coefficients,
        x,
        "../polynomial_sha256/inputs/poly_constant.json",
    )?;

    println!("Generated poly_constant.json: f(3) = 5");
    Ok(())
}

/// Generate input for a linear polynomial: f(x) = ax + b
fn generate_linear_polynomial() -> Result<(), Error> {
    println!("Generating linear polynomial: f(x) = 2x + 2");

    let n = 2u32;
    let coefficients = vec![2u32, 2u32]; // f(x) = 2x +2
    let x = 4u32;

    create_input_file(
        n,
        coefficients,
        x,
        "../polynomial_sha256/inputs/poly_linear.json",
    )?;

    println!("Generated poly_linear.json: f(4) = 2*4 + 2 = 10");
    Ok(())
}

/// Generate input for a quadratic polynomial: f(x) = ax^2 + bx + c
fn generate_quadratic_polynomial() -> Result<(), Error> {
    println!("Generating quadratic polynomial: f(x) = x² + 2x + 1");

    let n = 3u32;
    let coefficients = vec![1u32, 2u32, 1u32]; // f(x) = x^2 + 2x + 1
    let x = 3u32;

    create_input_file(
        n,
        coefficients,
        x,
        "../polynomial_sha256/inputs/poly_quadratic.json",
    )?;

    println!("Generated poly_quadratic.json: f(3) = 1*3² + 2*3 + 1 = 16");
    Ok(())
}
fn main() -> Result<(), Error> {
    std::fs::create_dir_all("../polynomial_sha256/inputs").map_err(|e| Error::Io(e))?;

    generate_constant_polynomial()?;
    generate_linear_polynomial()?;
    generate_quadratic_polynomial()?;
    Ok(())
}
