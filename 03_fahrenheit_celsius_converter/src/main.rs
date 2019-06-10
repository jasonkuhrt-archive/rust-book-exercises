use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("args were: {:?}", args);

    if args.len() != 3 {
        eprintln!("Please pass in a celsius or fahrenheit value to convert.\n\n  Examples:\n\n  -c 50\n  -f 10");
        panic!("Invalid command line arguments");
    }

    let flag = &args[1];
    let value: f64 = args[2].parse().expect("Invalid number");

    match flag.as_ref() {
        "-f" => println!("{}", (value - 32 as f64) / 1.8),
        "-c" => println!("{}", (value * 1.8) + 32 as f64),
        _ => println!("No such flag \"{}\"\n\nValid flags are: -c -f", flag),
    }
}
