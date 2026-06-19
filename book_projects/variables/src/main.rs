fn main() {
    let mut arr: [i32; 5] = [3; 5];

    arr[0] = 6;

    println!("{}", arr[0]);

    println!("Hello, world!");

    let value: f64 = (30.5 * 0.5) / (2.2 + 1.2);

    println!("the value = {value}");

    another_function();

    if (value > 5.0 && value < 7.0) || (value != 0.0) {
        println!("true")
    }

    for number in 1..4 {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    //Convert temperatures between Fahrenheit and Celsius.

    let celsius = 2.0;

    let fahrenheit: f64 = (celsius * (9.0 / 5.0)) + 32.0;

    println!("the celcius value is {celsius}, fahrenheit is {fahrenheit}");

    //Generate the nth Fibonacci number

    let n = 15;

    if n <= 0 {
        println!("Fibonacci is 0");
    } else if n == 1 {
        println!("Fibonacci is 1");
    } else {
        let mut first = 0;
        let mut second = 1;

        for _ in 0..n {
            println!("{first}");

            let next = first + second;

            second = first;
            first = next;
        }

        println!();
    }

    //Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

    let days: [&str; 8] = [
        "first", "second", "third", "fourth", "fiveth", "sixth", "seventh", "eigth",
    ];

    let lyrics: [&str; 8] = [
        "A partridge in a pear tree",
        "Two turtles drove and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six goose a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
    ];

    for (index, day) in days.into_iter().enumerate() {
        println!("On the {day} day Christmas, my true love sent to me");

        for lyric_index in (0..index + 1).rev() {
            println!("{}", lyrics[lyric_index])
        }

        println!();
    }
}

fn another_function() {
    println!("Another function.");
}
