use std::io::{self, Read, Write};

pub fn read_byte_from_stdin() -> u8 {
    read_byte(&mut io::stdin())
}

pub fn read_byte<R: Read>(reader: &mut R) -> u8 {
    let mut buf = [0];
    reader.read_exact(&mut buf);
    buf[0]
}

pub fn read_char<R: Read>(reader: &mut R) -> char {
    read_byte(reader) as char
}

/// Returns a trimmed line read from the input.
/// Note: Inefficient as it reads only one byte at a time.
pub fn read_line<R: Read>(reader: &mut R) -> String {
    let mut input_text = String::new();
    loop {
        let ch = read_char(reader);
        if ch == '\n' {
            break;
        } else {
            input_text.push(ch);
        }
    }
    let trimmed = input_text.trim();
    trimmed.to_string()
}

pub fn write<W: Write>(writer: &mut W, msg: &str) {
    writer.write_all(msg.as_bytes());
    writer.flush();
}

pub fn write_ln<W: Write>(writer: &mut W, msg: &str) {
    let msg_with_ln = msg.to_string() + "\n";
    write(writer, &msg_with_ln);
}
