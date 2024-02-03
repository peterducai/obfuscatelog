use obfuscatelog::reader;

const VERSION: &str = "0.1.0";
fn main() -> std::io::Result<()> {
    println!("obfuscatelog version {}", VERSION);
    let mut myreader = reader::reader::BufReader::open("testfile.txt")?;
    let mut buffer = String::new();

    while let Some(line) = myreader.read_line(&mut buffer) {
        println!("{}", line?.trim());
    }

    Ok(())
}