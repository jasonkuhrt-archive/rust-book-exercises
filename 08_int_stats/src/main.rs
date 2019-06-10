use std::collections::HashMap;

#[derive(Debug)]
struct Stats {
    mean: f64,
    median: u32,
    mode: (u32, u32),
}

fn main() {
    println!("Hello!");
    println!("Please enter a space-delimited list of integers.");
    println!("In return I shall present you some stats about them.");

    let mut ints: Vec<u32>;
    loop {
        ints = parse_ints(&get_input());
        if ints.len() > 0 {
            break;
        } else {
            println!("Please enter at least one integer");
        }
    }

    // Calculate the average
    let mean = sum_ints(&ints) as f64 / ints.len() as f64;

    // Calculate the median
    ints.sort();
    let median = *ints.get(ints.len() / 2).unwrap_or(&0);

    // Calculate the mode
    let mut counts: HashMap<u32, u32> = HashMap::new();

    for int in ints {
        counts.entry(int).and_modify(|int| *int += 1).or_insert(1);
    }

    let mut mode = (0, 0);
    for (int, count) in counts {
        if count > mode.1 {
            mode = (int, count);
        }
    }

    println!("Your stats are: {:?}", Stats { mean, median, mode })
}

fn get_input() -> String {
    let mut ints_input = String::new();
    std::io::stdin()
        .read_line(&mut ints_input)
        .expect("Failed to read line");
    ints_input
}

fn parse_ints(ints_input: &String) -> Vec<u32> {
    let mut ints: Vec<u32> = Vec::new();
    for int_input in ints_input.split_whitespace() {
        let int: u32 = match int_input.parse() {
            Ok(int) => int,
            Err(_) => {
                println!("Invalid integer given: \"{}\" (skipping)", int_input);
                continue;
            }
        };
        ints.push(int);
    }
    println!("Processed the given values as: {:?}", ints);
    ints
}

fn sum_ints(ints: &Vec<u32>) -> u32 {
    let mut sum = 0 as u32;
    for int in ints {
        sum += int;
    }
    sum
}
