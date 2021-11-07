use regex::Regex;

fn add(items: &str) -> i32 {
    let mut sum: i32 = 0;
    let s;
    let delim_slice;
    if items.starts_with("//") {
        let idx = items.find("\n").expect("Newline missing");
        delim_slice = items.get(2..idx).expect("Delimiter(s) missing");
        s = String::from(items.get(idx..items.len()).expect("Number slice out of range"));
    } else {
        delim_slice = ",\n";
        s = String::from(items);
    }

    if s != "" {
        let delim_string = format!("[{}]", delim_slice);
        let re = Regex::new(delim_string.as_str()).unwrap();
        let v: Vec<&str> = re.split(s.as_str()).collect();
        let mut negatives: Vec<i32> = Vec::new();
        for item in v {
            let candidate = item.trim().parse::<i32>().unwrap();
            if candidate < 0 {
                negatives.push(candidate);
            } 
            if candidate <= 1000 {
                sum += candidate;
            }
        }
        if !negatives.is_empty() {
            let mut error_msg = String::new();
            error_msg += format!("negatives not allowed {:?}", negatives).as_str();
            panic!("{}", error_msg);
        }
    }
    sum
}

fn main(){
    println!("{}", add("//;\n1;2"));
}

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_add_empty_string() {
        assert_eq!(0, add(""));
    }

    #[test]
    fn test_add_one() {
        assert_eq!(1, add("1"));
    }

    #[test]
    fn test_add_two_and_two() {
        assert_eq!(4, add("2, 2"));
    }

    #[test]
    fn test_binomial_add() {
        assert_eq!(32, add("1, 5,10,10,5,1"));
    }

    #[test]
    fn test_add_with_newline() {
        assert_eq!(32, add("1\n 5\n10,10,5,1"));
    }

    #[test]
    fn test_single_custom_delimiter() {
        assert_eq!(3, add("//;\n1;2"));
    }

    #[test]
    fn test_multiple_custom_delimiters() {
        assert_eq!(6, add("//;,\n1;2,3"));
    }

    #[test]
    #[should_panic(expected = "negatives not allowed [-1, -2]")]
    fn test_negative_numbers() {
        add("-1, -2");
    }

    #[test]
    fn test_numbers_bigger_than_1000_ignored() {
        assert_eq!(2, add("2, 1001"));
    }
}
