use std::io;
pub fn find_odd_in_arr() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    
    let mut arr = Vec::new();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    arr = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    
    if arr[0] != arr[1] {
        println!("{}", arr[0]);
    } else if arr[n - 1] != arr[n - 2] {
        println!("{}", arr[n - 1]);
    } else {
        let (mut left, mut right) = (0, n - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            let pre = if mid > 0 { mid - 1 } else { 0 };
            let nxt = if mid < n - 1 { mid + 1 } else { n - 1 };

            if arr[pre] != arr[mid] && arr[nxt] != arr[mid] {
                println!("{}", arr[mid]);
                break;
            }
            
            if mid % 2 == 0 {
                if arr[pre] == arr[mid] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if arr[pre] == arr[mid] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
    }
}