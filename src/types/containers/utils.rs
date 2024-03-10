/// [3,3,3,[3,3,3,3]]
/// 
/// (3,2,5,(3,5,6))
/// 
/// (3, "[\d.*,]*,*")
/// 
pub struct NestedValueParser {
    
}

impl NestedValueParser {

    pub fn parse_nested_str(input: &str) -> Vec<String> {
        // we start with either a value | [ | ( |  " for regex
        let mut values = vec![];
        let mut input = input.trim().to_string();
        // start walking through the string
        match input.chars().nth(0).unwrap() {
            '(' | '[' | '"' => {input.remove(0); input.pop();}, // clear the value wrapper () [] ""
            _ => {},
        };
        let mut recorded_chars = String::new();
        let mut inner_opened = false;
        let mut previous: char = input.chars().nth(0).unwrap();
        let mut is_escaped: bool;
        let mut recorded_only_whitespace = true;
        for ch in input.chars() {
            if previous == '\\' {is_escaped = true} else {is_escaped = false}
            if ch == ',' && !inner_opened && !recorded_only_whitespace {
                values.push(recorded_chars.trim().to_string());
                recorded_chars = String::new();
                continue
            }
            if !is_escaped {
                if ch == '(' || ch == '[' || ch == '"' {
                    inner_opened = true;
                    if !recorded_chars.is_empty() && !recorded_only_whitespace {
                        values.push(recorded_chars.trim().to_string());
                        recorded_chars = String::new();
                    } 
                }
                if ch == ')' || ch == ']' || ch == '"' {
                    inner_opened = false;
                }
            }
            previous = ch;
            if ch == ' ' {
                recorded_only_whitespace = true;
            } else {
                recorded_only_whitespace = false;
            }
            recorded_chars.push(ch);
        }
        if !recorded_chars.is_empty() && !recorded_only_whitespace {
            values.push(recorded_chars.trim().to_string());
        }        
        println!("{:?}", values);
        values
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ RType, Reference, ContainerType}; //NumericType, StringType, SpecialType,

    #[test]
    fn test_util_parse_nested_values() { 
        let input = "(3,2,5,(3,5,6),6)";
        let result = NestedValueParser::parse_nested_str(input);
        let expected: Vec<&str> = vec!["3","2","5","(3,5,6)", "6"];
        assert_eq!(expected, result);

        let input = "(usize,str,regex,(Vec<str>,Vec<Vec<str>>,gandalf),6)";
        let result = NestedValueParser::parse_nested_str(input);
        let expected: Vec<&str> = vec!["usize","str","regex","(Vec<str>,Vec<Vec<str>>,gandalf)", "6"];
        assert_eq!(expected, result);

        let input = "(&'static str, usize)";
        let result = NestedValueParser::parse_nested_str(input);
        let expected: Vec<&str> = vec!["&'static str","usize"];
        assert_eq!(expected, result);
    }

}
