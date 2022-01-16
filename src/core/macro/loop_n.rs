macro_rules! loopn {
    ($amount: expr, $body: block) => {
        for _ in 0..$amount
            $body
    };
}

#[test]
fn test_loopn() {
    loopn!(3,{
        println!("Hello Eugene Liu");
    })
}