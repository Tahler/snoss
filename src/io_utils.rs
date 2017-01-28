use std::io::{self, Read, BufRead, stdin};

pub fn read_byte_from_stdin() -> u8 {
    let input: Option<u8> = stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u8);
    input.unwrap()
}

/// Returns a trimmed line read from the input.
pub fn read_line() -> String {
    let mut input_text = String::new();
    let stdin = stdin();
    stdin.lock()
        .read_line(&mut input_text)
        .expect("failed to read from input");
    let trimmed = input_text.trim();
    trimmed.to_string()
}

pub fn write(msg: &str) {
    use std::io::Write;
    let mut output = io::stdout();
    output.write_all(msg.as_bytes());
    output.flush();
}

pub fn write_ln(msg: &str) {
    let msg_with_ln = msg.to_string() + "\n";
    write(&msg_with_ln);
}
