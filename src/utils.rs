pub fn pad_to_length(input: String, length: usize) -> String {
    format!("{:<width$}", input, width = length)
}
pub fn leftpad_to_length(input: String, length: usize) -> String {
  format!("{:>width$}", input, width = length)
}
pub fn indent_string(input: String, width: usize) -> String {
    format!("{:>width$}{}","", input, width=width)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leftpad_cargo_publish_uninstall() {
        let stri = String::from("Pisse");
        let go = indent_string(stri, 4);
        println!("{}", go);
    }
}
