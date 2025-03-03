
#[derive(Debug,PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|x: &Shoe| x.size == shoe_size)
        .collect()
}

fn main() {}

#[test]
fn filter_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("Boot"),
        },
        Shoe {
            size: 10,
            style: String::from("Casual"),
        },
        Shoe {
            size: 11,
            style: String::from("Formal"),
        },
    ];

    let in_my_shoes = shoes_my_size(shoes, 10);

    assert_eq!(
        in_my_shoes,
        vec![
            Shoe {
                size: 10,
                style: String::from("Boot")
            },
            Shoe {
                size: 10,
                style: String::from("Casual")
            },
        ]
    );
}
