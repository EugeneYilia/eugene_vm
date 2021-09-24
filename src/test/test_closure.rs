#[test]
fn test_closure() {
    println!("test_closure");

    let mut i = 1_i8;

    let mut f = move || {
        i += 1;
        i
    };

    let v = f();
    let v2 = f();

    i += 10;

    println!("{}", v);
    println!("{}", v2);
    println!("{}", i);
}