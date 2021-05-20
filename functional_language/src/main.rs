use std::thread;
use std::time::Duration;

struct Cacher<T> where T: Fn(u32) -> u32, {
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T> where T: Fn(u32) -> u32, {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {calculation, value: None}
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
/* 
    let buffer: &mut [i32];
    let coefficients: [i64; 12];
    let qlp_shift: i16;

    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
                                      .zip(&buffer[i - 12..i])
                                      .map(|(&c, &s)| c * s as i64)
                                      .sum::<i64>() >> qlp_shift;
    } */

}


fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
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

// Testing Cacher implementation
#[test] // it fails
fn call_with_different_value() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(2);
    let v2 = c.value(3);

    assert_eq!(v2, 3);
}
