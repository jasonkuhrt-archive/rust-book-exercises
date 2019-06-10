use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!(
            "Must provide exactly one integer argument denoting how far to calculate fib up to."
        );
        panic!("Invalid arguments");
    }

    let target_nth: u32 = args[1].parse().expect("Invalid positive integer");

    if target_nth == 0 {
        return println!("{}", 0);
    }

    let mut current_nth = 0;
    let mut prev_2 = 0;
    let mut prev_1 = 1;

    while current_nth < target_nth {
        let next = prev_2 + prev_1;
        prev_2 = prev_1;
        prev_1 = next;
        current_nth += 1;
    }

    println!("{}", prev_1);
}
