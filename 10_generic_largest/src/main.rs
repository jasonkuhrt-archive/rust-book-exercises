fn largest<T: PartialOrd>(xs: &[T]) -> &T {
    let mut largest = &xs[0];
    for x in xs.iter() {
        if x > largest {
            largest = x
        }
    }
    largest
}

fn main() {
    let numbers = vec![10, 5, 3, 8, 1, 13];
    println!("The largest number is: {}", largest(&numbers));

    let words = vec!["bar", "qux", "yol", "foo"];
    println!("The largest word is: {}", largest(&words));

    let chars = vec!["c", "q", "f", "y"];
    println!("The largest char is: {}", largest(&chars));
}
