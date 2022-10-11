fn main() {
    // let mut s = String::new();

    // let data = "initial contents";
    // let s = data.to_string();
    // let s = "initial contents".to_string();

    let s = String::from("initial contents");

    println!("{}", s);

    let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    println!("{}", hello);

    let mut f = String::from("foo");
    f.push_str("bar");
    println!("{}", f);

    let mut l = String::from("lo");
    l.push('l');
    println!("{}", l);

    let s1 = String::from("Peanut");
    let s2 = String::from("butter");
    let s3 = s1 + &s2;
    // s1, no longer valid.
    // s2, is deref coerced into &str
    println!("{}", s3);

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");
    // let ttt = t1 + "-" + &t2 + "-" + &t3;
    // After addition t1 is not available for use
    let ttt = format!("{}-{}-{}", t1, t2, t3);
    // After format! t1, t2, t3 are available for use
    println!("{}", ttt);
}
