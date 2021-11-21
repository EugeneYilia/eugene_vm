macro_rules! loopn {
    ($n:expr,$body:block) => {
        for _ in 0..$n
            $body
    };
}

#[test]
fn test_loopn() {
    loopn!(3,{
        println!("Hello Eugene Liu");
    })
}