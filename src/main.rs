use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let input = io::stdin().lock();
    let mut output = io::stdout().lock();
    for byte in input.bytes() {
        let byte = byte?;
        if byte.is_ascii() {
            output.write_all(&[byte])?;
        }
    }
    Ok(())
}
