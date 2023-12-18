use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() -> std::io::Result<()> {
    // let file = File::open("test_input.txt")?;
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let res = combine_first_last_digits(line);
        sum += res;
    }
    println!("{sum}");

    Ok(())
}

fn combine_first_last_digits(mut str: String) -> i32 {
    println!("before: {str}");

    let mut new_str = String::new();
    // let str_clone = str.clone();
    // let caps = re.captures_iter(str_clone.as_str()).collect::<Vec<_>>();
    // for cap in caps.iter().rev() {
    //     if let Some(cap_match) = cap.get(0) {
    //         let range = cap_match.range();
    //         let replace_with = match cap_match.as_str() {
    //             "one" => "1",
    //             "two" => "2",
    //             "three" => "3",
    //             "four" => "4",
    //             "five" => "5",
    //             "six" => "6",
    //             "seven" => "7",
    //             "eight" => "8",
    //             "nine" => "9",
    //             _ => "",
    //         };
    //         str.replace_range(range, replace_with);
    //     }
    // }

    let mut curr_word = String::new();
    for &b in str.as_bytes() {
        if b >= b'0' && b <= b'9' {
            new_str.push(b as char);
            curr_word.clear();
        } else {
            curr_word.push(b as char);

            let mut found = false;
            if curr_word.ends_with("one") {
                new_str.push('1');
                found = true;
            } else if curr_word.ends_with("two") {
                new_str.push('2');
                found = true;
            } else if curr_word.ends_with("three") {
                new_str.push('3');
                found = true;
            } else if curr_word.ends_with("four") {
                new_str.push('4');
                found = true;
            } else if curr_word.ends_with("five") {
                new_str.push('5');
                found = true;
            } else if curr_word.ends_with("six") {
                new_str.push('6');
                found = true;
            } else if curr_word.ends_with("seven") {
                new_str.push('7');
                found = true;
            } else if curr_word.ends_with("eight") {
                new_str.push('8');
                found = true;
            } else if curr_word.ends_with("nine") {
                new_str.push('9');
                found = true;
            }

            if found {
                // clear all but last character
                let c = curr_word.chars().last();
                curr_word.clear();
                if let Some(c) = c {
                    curr_word.push(c);
                }
            }
        }
    }

    str = new_str;

    println!("after: {str}");

    let mut res_str = String::new();

    // let mut idx_found = 0;
    for &b in str.as_bytes() {
        if b >= b'0' && b <= b'9' {
            res_str.push(b as char);
            break;
        }
    }

    for &b in str.as_bytes().iter().rev() {
        if b >= b'0' && b <= b'9' {
            res_str.push(b as char);
            break;
        }
    }

    res_str.parse().unwrap()
}
