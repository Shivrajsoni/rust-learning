use std::fmt::Display;

fn largest_str<'a,T>(
x:&'a str,
y:&'a str,
ann:T
) -> &'a str where
T:Display,
{
    println!("Announcment {}",ann);
    if x.len()>y.len(){
        x
    }else{
        y
    }
}

fn main(){
    let str1 = String::from("shivraj");
    let str2 = String::from("sidesh");
    
    let announcement = "Comparing strings";
    let l1 = largest_str(&str1, &str2, announcement);
    println!("{l1}");
}