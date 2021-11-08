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
// 导出宏的方法

// 1. mod顺序   宏的引用在mod.rs的声明有顺序要求   (1。测试是否方法导出  调用引用也有顺序需求(测试结果方法不需要)  2。是否只是宏才有顺序要求(测试结果只有宏才需要))
// 比如class_loader包需要使用annotation包里面的注解
// #[macro_use]
// pub mod macro;
// pub mod class_loader;
// 需要在core的mod.rs按照上述顺序进行声明  才可以在class_loader包内引用到annotation包内定义的宏
//
// 经测试 方法的跨模块导出  不需要遵循声明的先后顺序  只有宏的导出需要按照模块的依赖关系 定好顺序 进行声明 才可使用
// 2. 所有mod都需要#[macro_use]
// 首先在annotation的包里面需要加上
// #[macro_use]
// pub mod loop_n;
// 然后再core的包里面加上
// #[macro_use]
// pub mod macro;
// 以后在对应的文件里
// use crate::core::macro::loop_n;
// 就可以使用该宏了




// ---------




// Example 4:
//