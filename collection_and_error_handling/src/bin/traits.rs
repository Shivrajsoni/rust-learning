fn largest_val<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let v = vec!['a', 'b', 'c'];
    let largestvalue = largest_val(&v);
    println!("Largest value is - {}", largestvalue);
}
