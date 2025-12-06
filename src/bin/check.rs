use {
    std::{
        env,
        fs::File,
        io::{Read}
    },
    colored::Colorize
};

const TEST_DATA_DIR: &str = "./data/";

fn get_file(file_name: String) -> Option<File> {
    let file_path = String::from(TEST_DATA_DIR) + &file_name;
    match File::open(&file_path) {
        Ok(file) => Some(file),
        Err(error) => {
            println!("Error opening file \"{}\": {}", file_path, error);
            None
        }
    }
}

/// Remove Byte Order Mark for utf-8
fn remove_utf8_bom(s: &str) -> &str {
    const UTF8_BOM: &[u8] = &[0xEF, 0xBB, 0xBF];
    if s.as_bytes().starts_with(UTF8_BOM) { &s[UTF8_BOM.len()..] } else { s }
}

fn get_tokens(file: &mut File) -> Vec<String> {
    let mut content: String = String::new();
    file.read_to_string(&mut content).unwrap_or_else(|err| {
        panic!("Error reading file content: '{}'", err);
    });
    remove_utf8_bom(&content).split_whitespace().map(|s| s.to_lowercase()).collect()
}

fn validate(test_case: &String) -> Option<()> {
    println!("Validating test case {}", test_case);
    let expected_file = test_case.clone() + &String::from(".ans");
    let output_file = test_case.clone() + &String::from(".out");
    let mut expected_file = get_file(expected_file)?;
    let mut output_file = get_file(output_file)?;
    println!("reading expected file");
    let expected = get_tokens(&mut expected_file);
    println!("reading output file");
    let output = get_tokens(&mut output_file);
    if expected.len() != output.len() {
        panic!("Expected {} tokens but found {}", expected.len(), output.len());
    }
    for i in 0..expected.len() {
        if expected[i] != output[i] {
            panic!("{}-th token did not match. Expected \"{}\" but got \"{}\"", i, expected[i], output[i]);
        }
    }
    println!("{}", format!("-> Correct! {} tokens match!", expected.len()).green().bold());
    Some(())
}

fn main() {
    println!("Running check.rs...");
    let args: Vec<String> = env::args().collect();
    for test_case in &args[1..] {
        validate(test_case).unwrap(); // TODO: handle nicely
    }
}
