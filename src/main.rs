use std::{
    env, fs,
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if let Some(file_path) = args.get(1) {
        let file = fs::File::open(file_path)?;
        let lines = io::BufReader::new(file).lines();
        lines.flatten().for_each(|x| println!("{x}"));
    } else {
        let stdin = io::stdin();
        let lines = stdin.lines();
        lines.flatten().for_each(|x| println!("{x}"));
    }

    Ok(())
}
