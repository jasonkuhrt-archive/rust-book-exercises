fn main() {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let ordinal_english_numbers = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let mut previous_gifts: Vec<String> = std::vec::Vec::new();

    for (i, gift) in gifts.iter().enumerate() {
        let opening = format!(
            "On the {} day of Christmas,\nmy true love sent to me",
            ordinal_english_numbers[i]
        );
        previous_gifts.splice(0..0, [gift.to_string()].iter().cloned());
        println!("{}\n{}\n", opening, previous_gifts.join(",\n").trim());

        if i == 0 {
            let first_gift = previous_gifts
                .pop()
                .expect("First gift not added to previous state yet");
            previous_gifts.push(format!("And a{}.", first_gift.split_at(1).1.to_string()));
        }
    }
}
