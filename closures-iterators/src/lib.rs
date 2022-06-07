#[test]
fn iterator_demo() {
    let vec_1 = vec![1, 2, 3];

    // note we make the iterator mutable because as we call next,
    // the internal state the iterator uses to track where it is changes.
    // it 'eats it up' aka a consuming adaptor.
    let mut vec_iter = vec_1.iter();

    assert_eq!(vec_iter.next(), Some(&1));
    assert_eq!(vec_iter.next(), Some(&2));
    assert_eq!(vec_iter.next(), Some(&3));
    assert_eq!(vec_iter.next(), None)
}

#[test]
fn iterator_sum() {
    let vec_1 = vec![1, 2, 3];
    let vec_iter = vec_1.iter();

    // also a consuming adaptor.
    // behind the scenes, sum repeatedly calls next
    let sum: i32 = vec_iter.sum();

    assert_eq!(sum, 6);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// takes ownership of a vector and a shoe size
// into_iter takes ownership of the vector
// capture size into our filter closure
fn shoes_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let all_shoes: Vec<Shoe> = vec![
            Shoe {
                size: 11,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size: Vec<Shoe> = shoes_in_size(all_shoes, 11);

        assert_eq!(
            in_my_size,
            vec![Shoe {
                size: 11,
                style: String::from("sneaker"),
            },]
        )
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
}

struct Counter {
    // private count by default - let implementation manage it
    count: u32,
}

impl Counter {
    // our constructor will always initialize a new struct with a count of 0
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// to implement an interator for our Counter struct we need to define the next function
impl Iterator for Counter {
    type Item = u32;
    // signature takes a mutable reference and returns an Option
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
