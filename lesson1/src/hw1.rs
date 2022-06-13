fn check_subset(org_arr: &[i32], sub_arr: &[i32]) -> i8 {
    if sub_arr.len() > org_arr.len(){
        return 0;
    } 
    if sub_arr.len() == org_arr.len() && sub_arr[0] != org_arr[0] {
        return 0;
    } 
    let x = sub_arr[0];
    let mut start = 0;
    for i in 1..org_arr.len() {
        if org_arr[i] == x {
            start = i;
            break;
        }
    }
    if start == 0 && org_arr[0] != sub_arr[0] {
        return 0;
    }
    for i in 0..sub_arr.len() {
        if org_arr[start + i] != sub_arr[i] {
            return 0;
        }
    }
    return 1;
}

fn main() {
    let org_arr:[i32; 8] = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr:[i32; 3] = [6, 8, 10];
    let result:i8 = check_subset(&org_arr, &sub_arr);
    println!("{}", result);
}
