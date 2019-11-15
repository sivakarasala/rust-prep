pub fn vector_type() {
    let v: Vec<i32> = Vec::new();
    println!("Shiva {:?}", v);

    let v = vec![1, 2, 3];
    println!("Shambho {:#?}", v);

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Shankara {:#?}", v);

    // Reading elements of Vectors
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100'
    let does_not_exist = v.get(100);
    println!("hara hara mahadeva {:?}", does_not_exist); // Return None

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    //v.push(6);

    println!("The first element is: {} {:#?}", first, v);

    // Iterating over Vector
    let v = vec![32, 45, 67, 43, 25, 89, 26];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    for i in &mut v {
        *i *= 2;
    }
    println!("Squares is {:#?}", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Aum namah shivaya")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Vector : {:?}", row);
}