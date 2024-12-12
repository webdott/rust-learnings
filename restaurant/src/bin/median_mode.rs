use std::collections::HashMap;

fn mode(list: &Vec<i32>) -> i32 {
    let mut hash: HashMap<i32, i32> = HashMap::new();
    let mut max_occ_num = -1;
    let mut max_freq = 0;

    for num in list.into_iter() {
        let count = hash.entry(*num).or_insert(0);
        *count += 1;

        if *count > max_freq {
            max_freq = *count;
            max_occ_num = *num
        }
    }

    return max_occ_num;
}


fn median(list: &mut Vec<i32>) -> i32 {
    let mut_list = &mut list[..];

    mut_list.sort();

    let list_length = mut_list.len();

    if list_length % 2 == 0 {
        return (mut_list[(list_length - 1) / 2] + mut_list[list_length / 2]) / 2;
    } else {
        return mut_list[list_length / 2]
    }
}

use std::collections::HashSet;

fn to_pig_latin(string_to_convert: &str) -> String {
    let vowels: HashSet<char> = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    let mut res_str = String::new();

    for word in string_to_convert.split_whitespace() {
        let first_ele = word.chars().nth(0);

        match first_ele {
            Some(value) => {
                if vowels.contains(&value) {
                    res_str.push_str(&format!("{word}hay "));
                } else {
                    res_str.push_str(&format!("{}{}ay ", &word[1..], value));
                }
            },
            None => return String::new()
        }
    }

    res_str.trim_end().to_string()
}

fn main() {
    let mut integer_list = vec![1, 2, 3, 2, 2, 5, 7, 8, 7, 3, 2, 7, 7];

    let median_value = median(&mut integer_list);

    let mode_value = mode(&integer_list);

    println!("median value: {:?}, mode value: {:?}", median_value, mode_value);

    let base_string = "Uche is a boy";

    let converted_string = to_pig_latin(&base_string);

    println!("{converted_string:?}")
}


