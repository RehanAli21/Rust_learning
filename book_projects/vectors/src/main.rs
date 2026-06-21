fn main() {
    let v1: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // iterating for read only
    for i in &v {
        println!("{i}");
    }

    // iterating for read and write
    for i in &mut v {
        *i += 50;
    }

    //reading vectors
    let v3 = vec![1, 2, 3, 4, 5];

    // compiler panic on out of bound index
    // use to crash program
    let third: &i32 = &v3[2];
    println!("The third element is {third}");

    // get None on out of bound index
    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // error because of ownership
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");

    // with enums
    //
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // now this vector can only have value which are present in SpreadsheetCell enum
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
