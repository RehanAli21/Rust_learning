fn main() {
    //creating new strings
    //
    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // when to use String::from
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    //updating string
    //
    let mut s = String::from("foo");
    s.push_str("bar");

    // update string without ownership
    // push_str gets string slice
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    //add char
    let mut s = String::from("lo");
    s.push('l');

    //concat string
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
                       // + operator = fn add(self, s: &str) -> String

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{s}");

    // to iterate in string have to use .chars() because the string is save in UTF-8 in book they
    // defined how one char can be save in multiple bytes
    for c in s.chars() {
        println!("{c}");
    }
}
