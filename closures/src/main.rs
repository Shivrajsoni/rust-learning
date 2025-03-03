//fn main() {
//let s = String::from("hello");
//let borrow_closure = || println!("{}", s);
// borrow_closure();
//let multiply = |x| x * factor;

//let mut count = 0;
//let mut inc = || count += 1;
//inc();
//  println!("{}", count);
//}

//Rust categorizes closures based on how they capture variables:
//	•	Fn → Borrow variables (&T)
//	•	FnMut → Mutably borrow (&mut T)
//	•	FnOnce → Take ownership (T)

fn call_twice<F: Fn(i32) -> i32>(f: F) {
    println!("{}", f(2));
    println!("{}", f(3));
}

fn main() {
    let factor = 3;
    call_twice(|x| x * factor);
}
