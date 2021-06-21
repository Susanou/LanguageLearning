use std::collections::HashMap;

fn main() {
    let _v: Vec<i32> = Vec::new(); // new vector
    let _v = vec![1,2,3];

    // updating said vector

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);


    {
        let _v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    //iteration over a vector
    let v = vec![100, 32, 57];
    for i in v{
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v{
        *i += 50;
    }
    for i in v{
        println!("{}", i);
    }


    //Everything in a vec  needs to be of the same type. To make it store different types, we can use an enum
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(20.15),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    //UTF-8 encoding with strings

    let data = "initial content";
    let _s = data.to_string();

    //The literal also has the same function
    let _s = "initial content".to_string();

    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    //String is essentially the same as a Vec but for text
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    //However, strings don't support indexing like vectors
    //instead you should turn the string into list of bytes or chars
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    //Hashmaps 

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // if Blue didn't exist it would insert but since it does it doesn't replace the value

    println!("{:?}", scores);
    
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; //since count is a reference to the value in the hashmap we are modifying the actual value inside of the hashmap
    }

    println!("{:?}", map);

}
