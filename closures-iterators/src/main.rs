use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    closure_func: T,
    value: Option<u32>,
}

fn main() {
    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        // returns a Cacher instance that holds our closure function
        fn new(closure_func: T) -> Cacher<T> {
            Cacher {
                closure_func,
                value: None,
            }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.closure_func)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // another example:
    let x = vec![1, 2, 3];
    // to force the closure to take ownership of the values it uses in the environment,
    // you can use the move keyword before the parameter list.
    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x); // this will err out: cannot borrow a value after a move

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));

    let v1: Vec<i32> = vec![1, 2, 3];

    // because iterators are lazy, while creating the map closure function, we still need to call it
    let vec2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(vec2, [2, 4, 6])
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        // instead of calling the closure directly from Cacher, we call the value
        // if the value doesn't exist, it will call closure_func with our provided arg and set the value
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        // the next time we call it, we won't have to re-execute the closure function ("calculating slowly..." should only appear once!)
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
