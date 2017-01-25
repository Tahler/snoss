pub fn read_byte_from_stdin() -> u8 {
    use std::io::{Read, stdin};
    let mut buf = [0];
    stdin().read_exact(&mut buf);
    buf[0]
}
