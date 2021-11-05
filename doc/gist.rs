// Example 1:
fn build_classpath_entry(boot_classpath: &str) -> ClasspathEntry {
    // Situation 1:
    let path =
        Path::new(boot_classpath)
            .join("lib")
            .join("*");
    // 需要使用中间变量path来承接一下PathBuf 如果直接to_str().unwrap()再用变量接  再传递给ClasspathEntry 会报超出生命周期的作用域
    // 如果不接 在对应调用结束后  分号那里会将对应的生命周期结束的变量进行回收  之后再进行传递就会超出生命周期  所以这种写法是可以完成变量的传递的

    let result = path.to_str()
        .unwrap();
    println!("{}", result);
    ClasspathEntry::new(
        result
    )

    // Situation 2:
    // 直接将对象不进行变量赋值  直接进行传递  此时变量生命周期还是存在的  可以进行成功传递
    // ClasspathEntry::new(
    //     Path::new(boot_classpath)
    //         .join("lib")
    //         .join("*")
    //         .to_str()
    //         .unwrap()
    // )
}





// ---------


// Example 2:
// 声明宏的定义 来实现loopn
macro_rules! loopn {
    ($n:expr,$body:block) => {
        for _ in 0..$n
            $body
    };
}

#[test]
fn test_loopn (){
    loopn!(3,{
        println!("Hello World");
    })
}




// ---------




// Example 3:
//