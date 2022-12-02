use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read},
    path::Path,
};

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(filepath);
    println!("{},{}", path.exists(), path.display());
    let data = fs::read_to_string(path).expect("expect a right directory");
    Ok(data)
}

fn read_file_vec(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = fs::read(filepath)?;
    Ok(data)
}

fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.expect("file line"));
    }

    Ok(())
}
// 文件读入 buffer
fn read_file_buffer(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    const BUFFER_LEN: usize = 512;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut file = File::open(filepath)?;

    loop {
        let read_count = file.read(&mut buffer)?;
        let buff = &buffer[..read_count];
        println!("{:?}",buff);

        if read_count != BUFFER_LEN {
            break;
        }
    }
    Ok(())
}

fn iterate_dir() {
    let target_path = Path::new("./");
    let entries = fs::read_dir(target_path).unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            println!("{}", entry.path().to_str().unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILEPATH: &str = "./src/practice/file_reader.rs";

    #[test]
    fn test_read_file_string() {
        let data = read_file_string(FILEPATH).unwrap();
        println!("{}", data);
    }
    #[test]
    fn test_read_file_vec() {
        let data = read_file_vec(FILEPATH).unwrap();
        // println!("{:?}",data);
    }
    #[test]
    fn test_read_file_line_by_line() {
        let data = read_file_line_by_line(FILEPATH).unwrap();
        // println!("{:?}",data);
    }
    #[test]
    fn test_read_file_buffer() {
        let data = read_file_buffer(FILEPATH).unwrap();
        println!("{:?}",data);
    }

    #[test]
    fn test_iterate_dir() {
        iterate_dir()
    }
}
