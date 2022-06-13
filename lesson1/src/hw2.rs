use std::io::stdin;

fn count_subset(org_str: &str) -> i8{
    let mut input_str = String::new();
    stdin().read_line(&mut input_str)
    	.ok()
        .expect("Failed to read line");
    // println!("{}, {}", org_str.chars().nth(0).unwrap(), input_string.chars().nth(0).unwrap());
    let mut count = 0;
    for mut i in 0..org_str.chars().count() {
        if org_str.chars().nth(i).unwrap() == input_str.chars().nth(0).unwrap() {
            for j in 0..input_str.chars().count()-1 {
                if org_str.chars().nth(i+j).unwrap() != input_str.chars().nth(j).unwrap() {
                    break;
                }
                if j == input_str.chars().count()-1 {
                    count += 1;
                }
                i += 1;
            }
        }
    }
    return count;
}

fn main() {
    let org_str = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.
    This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with
    the default style of Normal.";
    let result = count_subset(org_str);
    println!("{}", result);
}
