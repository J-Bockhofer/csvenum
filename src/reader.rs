use std::fs::File;
use std::io::{BufRead, BufReader, Result, Write};
pub fn read_file_lines(file_path: &str) -> Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    
    Ok(lines)
}

pub fn write_lines_to_file(file_path: &str, lines: Vec<String>) -> Result<()> {
    let mut file = File::create(file_path)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}