use obfuscatelog::reader::reader;
use obfuscatelog::collector::collector;
use obfuscatelog::matcher::matcher;

const VERSION: &str = "0.1.0.i23";


fn main() -> std::io::Result<()> {
    println!("obfuscatelog version {}", VERSION);

    let mut pool = collector::Collect::new();
    pool.push_domain("John".to_string());

    matcher::match_ipv4(String::from("127.0.0.1  dfskfl whasl.com 333.333.333.333 10.20.10.4"));

    let mut myreader = reader::BufReader::open("large_file.txt")?;
    let mut buffer = String::new();

    while let Some(line) = myreader.read_line(&mut buffer) {
        //println!("{}", line?.trim());
        let val = line.unwrap().trim();
        println!("{}", val);
        matcher::match_word(val, "specific_word");
    }

    Ok(())
}