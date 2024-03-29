#[allow(dead_code)]
pub fn pad_to_length(input: String, length: usize) -> String {
    format!("{:<width$}", input, width = length)
}
#[allow(dead_code)]
pub fn leftpad_to_length(input: String, length: usize) -> String {
  format!("{:>width$}", input, width = length)
}
pub fn indent_string<T: AsRef<str>>(input: T, width: usize) -> String {
    format!("{:>width$}{}","", input.as_ref(), width=width)
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