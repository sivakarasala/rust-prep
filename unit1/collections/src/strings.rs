pub fn string_type() {
    let mut s = String::new();

    let data = "aum namah shivaya";

    let s = data.to_string();

    // the method also works on a literal directly
    let s = "aum namah shivaya".to_string();

    let s = String::from("aum namah shivaya");


    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("shiva");
    s.push_str("shambho");

    let mut s1 = String::from("shiva ");
    let s2 = "shambho";
    s1.push_str(s2);
    println!("s2 is {} s is {}", s2, s);

    let mut s = String::from("shiv");
    s.push('a');

    let s1 = String::from("aum ");
    let s2 = String::from("namah ");
    let s3 = String::from("shivaya");
    let s4 = s1 + &s2 + &s3;
    println!("s4 is {}", s4);
}