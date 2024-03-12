/// [3,3,3,[3,3,3,3]]
/// 
/// (3,2,5,(3,5,6))
/// 
/// (sadsa ,"sadsa", dsadas)
/// 
pub struct NestedValueParser {
    
}

impl NestedValueParser {

    pub fn parse_nested_str(input: &str, csymbol: char, clear_self: bool) -> Vec<String> {
        // we start with either a value | [ | ( |  " for string
        let mut values = vec![];
        if input.is_empty() {return vec![]}
        let mut input = input.trim().to_string();
        // start walking through the string
        if clear_self {
            match input.chars().nth(0).unwrap() {
                c if c == csymbol => {input.remove(0); input.pop();}, // clear the value wrapper () [] "" '(' | '[' | '"'
                _ => {},
            };
        }     

        if input.is_empty() {return values}
        let mut recorded_chars = String::new();
        let mut inner_opened = false;
        let mut open_cnt:usize = 0;
        let mut previous: char = input.chars().nth(0).unwrap();
        let mut is_escaped: bool;
        let mut num_rec_whitespaces: usize = 0;

        let mut string_opened = false;


        for ch in input.chars() {
            if previous == '\\' {is_escaped = true} else {is_escaped = false}
            if ch == ',' && !inner_opened && !recorded_only_whitespace(&recorded_chars, num_rec_whitespaces) && open_cnt == 0  {
                // check if the segment was a string - if yes remove the quotes bc they are annoying and i dont wanna deal with them later.
                recorded_chars = recorded_chars.trim().to_string();
                let first = recorded_chars.chars().nth(0).unwrap();
                if first == '"' || first == '\"' {
                    recorded_chars.remove(0);
                    recorded_chars.pop();
                    recorded_chars = recorded_chars.trim().to_string();
                }
                values.push(recorded_chars);
                recorded_chars = String::new();
                num_rec_whitespaces = 0;
                continue
            }
            //println!("{} is escaped: {}",ch, is_escaped);
            //let pre_two: &str = previous + ch;
            if !is_escaped || ch == '"' {

                if ch == '"' || ch == '\"' {
                    string_opened = !string_opened;
                }

                if ch == '(' || ch == '[' || (string_opened && (ch == '"' || ch == '\"')) {
                    //println!("Opening closure");
                    open_cnt = open_cnt.saturating_add(1);
                    inner_opened = true;
                    // We have opened a new inner, check if an inner already exists, then we will treat it as part of the same valuestr
                    if !recorded_chars.is_empty() && !recorded_only_whitespace(&recorded_chars, num_rec_whitespaces) && open_cnt == 0 {
                        // Only push the recorded_chars as a whole valuestr when: 
                        // 1. it isnt empty
                        // 2. it does not only contain whitespaces
                        // 3. does not have any inner container opened
                        values.push(recorded_chars.trim().to_string());
                        recorded_chars = String::new();
                        num_rec_whitespaces = 0;
                    } 
                }
                if ch == ')' || ch == ']' || (!string_opened && (ch == '"' || ch == '\"')) {
                    //println!("Closing closure");
                    open_cnt = open_cnt.saturating_sub(1);
                    inner_opened = false;
                }
            }
            previous = ch;
            if ch == ' ' {
                num_rec_whitespaces = num_rec_whitespaces.saturating_add(1);
            } 
            recorded_chars.push(ch);
        }
        if !recorded_chars.is_empty() && !recorded_only_whitespace(&recorded_chars, num_rec_whitespaces) {
            recorded_chars = recorded_chars.trim().to_string();
            let first = recorded_chars.chars().nth(0).unwrap();
            if first == '"' || first == '\"' {
                recorded_chars.remove(0);
                recorded_chars.pop();
                recorded_chars = recorded_chars.trim().to_string();
            }

            values.push(recorded_chars);
        }        
        //println!("{:?}", values);
        values
    }
}

fn recorded_only_whitespace(rec_chars: &String, num_whitespace: usize) -> bool {
    if rec_chars.len() == num_whitespace {true} else {false}
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::{ RType, Reference, ContainerType}; //NumericType, StringType, SpecialType,

    #[test]
    fn test_util_parse_nested_values() { 
        let input = "(3 , 2 , 5 , (3,5,6) , 6 )";
        let result = NestedValueParser::parse_nested_str(input, '(', true);
        let expected: Vec<&str> = vec!["3","2","5","(3,5,6)", "6"];
        assert_eq!(expected, result);

        let input = "(usize,str,regex,(Vec<str>,Vec<Vec<str>>,gandalf),6)";
        let result = NestedValueParser::parse_nested_str(input, '(', true);
        let expected: Vec<&str> = vec!["usize","str","regex","(Vec<str>,Vec<Vec<str>>,gandalf)", "6"];
        assert_eq!(expected, result);

        let input = "(&'static str, usize)";
        let result = NestedValueParser::parse_nested_str(input, '(', true);
        let expected: Vec<&str> = vec!["&'static str","usize"];
        assert_eq!(expected, result);

        let input = "((usize, usize), usize)";
        let result = NestedValueParser::parse_nested_str(input, '(', true);
        let expected: Vec<&str> = vec!["(usize, usize)","usize"];
        assert_eq!(expected, result);

        let input = "(((usize, usize), usize), usize)";
        let result = NestedValueParser::parse_nested_str(input, '(', true);
        let expected: Vec<&str> = vec!["((usize, usize), usize)","usize"];
        assert_eq!(expected, result);

        let input = "[[[usize; 3]]]";
        let result = NestedValueParser::parse_nested_str(input, '[', true);
        let expected: Vec<&str> = vec!["[[usize; 3]]"];
        assert_eq!(expected, result);

        let input = "([[usize; 3]], usize)";
        let result = NestedValueParser::parse_nested_str(input, '(', true);
        let expected: Vec<&str> = vec!["[[usize; 3]]", "usize"];
        assert_eq!(expected, result);

        let input = "([[usize; 3]], usize)";
        let result = NestedValueParser::parse_nested_str(input, '(', true);
        let expected: Vec<&str> = vec!["[[usize; 3]]", "usize"];
        assert_eq!(expected, result);

        let input = "((usize, usize), usize, [usize; 3])";
        let result = NestedValueParser::parse_nested_str(input, '(', true);
        let expected: Vec<&str> = vec!["(usize, usize)", "usize", "[usize; 3]"];
        assert_eq!(expected, result);
    }
    #[test]
    fn test_parsed_nested_string() {

        let input = "3 , 2 , \"5 , (3,5,6) \", 6 ";
        let result = NestedValueParser::parse_nested_str(input, '\"', true);
        let expected: Vec<&str> = vec!["3","2","5 , (3,5,6)", "6"];
        assert_eq!(expected, result);

    }


}
