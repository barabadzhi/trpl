use std::str::FromStr;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    let mut list = Vec::new();
    let mut sum = 0i32;

    let mut list_hash = HashMap::new();

    for arg in std::env::args().skip(1) {
        match i32::from_str(&arg) {
            Ok(num) => {
                list.push(num);
                sum += num;

                let count = list_hash.entry(num).or_insert(0);
                *count += 1;
            }
            Err(..) => {}
        }
    }

    let list_len = list.len();

    if list_len == 0 {
        writeln!(std::io::stderr(), "Usage: list_of_integers NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mean = sum as f64 / list_len as f64;

    list.sort();
    let median = list[list_len / 2];

    let mut mode = (0, 0);

    for (num, count) in list_hash.iter() {
        if count > &mode.1 {
            mode = (*num, *count);
        }
    }

    println!("list: {:?}\nmean: {:.3}\nmedian: {:.3}\nmode: {}",
             list,
             mean,
             median,
             mode.0);
}
