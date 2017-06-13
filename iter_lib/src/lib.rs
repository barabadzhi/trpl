#[derive(PartialEq, Debug)]
pub struct Shoe {
    pub size: i32,
    pub style: String,
}

pub fn shoe_in_my_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![Shoe {
                         size: 10,
                         style: String::from("sneaker"),
                     },
                     Shoe {
                         size: 13,
                         style: String::from("sandal"),
                     },
                     Shoe {
                         size: 10,
                         style: String::from("boot"),
                     }];

    let in_my_size = shoe_in_my_size(shoes, 10);

    assert_eq!(in_my_size,
               vec![Shoe {
                        size: 10,
                        style: String::from("sneaker"),
                    },
                    Shoe {
                        size: 10,
                        style: String::from("boot"),
                    }]);
}

#[derive(Debug)]
pub struct Counter {
    pub count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}
