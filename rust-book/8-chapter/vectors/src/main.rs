fn main() {
    // let v = vec![1, 3, 1, 2];
    // let v: Vec<i32> = Vec::new();

    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(3);
    v.push(1);
    v.push(2);

    let third: &i32 = &v[2]; // Gives a reference to the element
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v {
        print!("{}", i);
        if v.last() == Some(i) {
            print!("\n");
        }
    }

    for i in &mut v {
        *i += 1936
    }

    for i in &v {
        println!("    {}", i);
    }
}
