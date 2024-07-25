use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let buffer = stdin.lines();
    buffer.flatten().for_each(|line| println!("{line}"));
    Ok(())
}
