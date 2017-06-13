use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T, K, V>
    where T: Fn(K) -> V,
          K: Eq + std::hash::Hash + Clone,
          V: Clone
{
    calculation: T,
    value: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
    where T: Fn(K) -> V,
          K: Eq + std::hash::Hash + Clone,
          V: Clone
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        let closure = &self.calculation;
        let v = self.value
            .entry(arg.clone())
            .or_insert_with(|| (closure)(arg));
        (*v).clone()
    }
}

fn generate_workout(intensity: i32, random_number: i32) {
    let mut expensive_result = Cacher::new(|num| {
                                               println!("calculating slowly...");
                                               thread::sleep(Duration::from_secs(2));
                                               num
                                           });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!("Today, run for {} minutes!",
                 expensive_result.value(intensity));
    }
}

#[test]
#[allow(unused_variables)]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
