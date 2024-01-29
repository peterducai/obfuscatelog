// fn main() -> std::io::Result<()> {
//     let mut reader = my_reader::BufReader::open("Cargo.toml")?;
//     let mut buffer = String::new();
//
//     while let Some(line) = reader.read_line(&mut buffer) {
//         println!("{}", line?.trim());
//     }
//
//     Ok(())
// }

pub mod reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        // to read large files re-use buffer for each line
        pub fn read_line<'buf>(
            &mut self,
            buffer: &'buf mut String,
        ) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}


One way to avoid the line buffering problem would be to wrap STDOUT in another, much larger buffer.

// fn main() {
//     let file = File::open("big_file.txt").unwrap();
//     let reader = BufReader::new(file);
//     let mut writer = BufWriter::new(std::io::stdout()):
//
//     for line in reader.lines() {
//         writeln!(writer, "{}", line.unwrap());
//     }
//     writer.flush().unwrap();
// }