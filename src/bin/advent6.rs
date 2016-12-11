// advent6.rs
// repetition codes

use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();

    let inputs: Vec<String> =
        stdin.lock().lines().map(|l| l.expect("Failed to read line")).collect();
    let messages: Vec<&str> = inputs.iter().map(|x| x.as_str()).collect();

    let corrected = correct_errors(&messages, false);
    println!("Part 1 corrected message: {}", corrected);

    let corrected2 = correct_errors(&messages, true);
    println!("Part 2 corrected message: {}", corrected2);
}

fn correct_errors(messages: &[&str], reverse_sort: bool) -> String {
    if messages.len() == 0 {
        return "".to_string();
    }

    let message_len = messages[0].len();
    let mut histograms = Vec::with_capacity(message_len);
    for _ in 0..message_len {
        histograms.push(HashMap::new());
    }

    for message in messages {
        for (c, mut histogram) in message.chars().zip(histograms.iter_mut()) {
            *histogram.entry(c).or_insert(0) += 1;
        }
    }

    let mut corrected = String::new();
    for histogram in histograms {
        let mut char_counts: Vec<(char, usize)> = histogram.iter().map(|(k, v)| (*k, *v)).collect();
        if reverse_sort {
            // part 2
            char_counts.sort_by(|a, b| a.1.cmp(&b.1));
        } else {
            // part 1
            char_counts.sort_by(|a, b| b.1.cmp(&a.1));
        }

        corrected.push(char_counts[0].0);
    }

    corrected
}

// ///////
// Tests

#[test]
fn test_correct_errors() {
    let v = vec!["eedadn", "drvtee", "eandsr", "raavrd", "atevrs", "tsrnev", "sdttsa", "rasrtv",
                 "nssdts", "ntnada", "svetve", "tesnvt", "vntsnd", "vrdear", "dvrsen", "enarar"];

    assert_eq!("easter", correct_errors(&v, false));
    assert_eq!("advent", correct_errors(&v, true));
}
