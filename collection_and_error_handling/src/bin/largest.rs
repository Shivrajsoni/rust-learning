fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

fn main() {
    let v = vec![100, 22, 3, 45, 2];
    let largestvalue = largest_i32(&v);
    println!("Largest value is -{}", largestvalue);

    let c = vec!['a', 'd', 'i', 'y'];
    let largestchar = largest_char(&c);
    println!("largest character is - {} ", largestchar);
}
