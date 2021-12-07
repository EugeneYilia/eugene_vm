use std::path::Path;

#[test]
fn test_is_exist() {
    let result = is_path_exist("eugene_test");
    println!("{}", result);//true
    let result = is_path_exist("eugene_test2");
    println!("{}", result);//false
    let result = is_path_exist("eugene_test/aaa");
    println!("{}", result);//false
    let result = is_path_exist("eugene_test/bbb");
    println!("{}", result);//true
    let result = is_path_exist("eugene_test/src_code/eugene_rt");
    println!("{}", result);//true
}

pub fn is_path_exist(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

pub fn is_dir(file_path: &str) -> bool {
    Path::new(file_path).is_dir()
}
