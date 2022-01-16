/// @arg  str 源字符串
/// @arg index 只读字符串中的具体的某个字符的位置索引
/// @return 具体的包含一个字符的只读字符串
macro_rules! indexn_str {
    ($str: expr, $index: expr) => {
        &$str[$index..$index + 1]
    }
}

/// @arg  str 源字符串
/// @arg index 只读字符串中的具体的某个字符的位置索引
/// @return 具体的一个字符
macro_rules! indexn_char {
    ($str: expr, $index: expr) => {
        $str.chars().nth($index).unwrap()
    }
}

#[test]
fn test_indexn_str() {
    let str = "abc";
    let x = indexn_str!(str, 1);
    println!("{}", x);
}

#[test]
fn test_indexn_char() {
    let str = "abc";
    let x = indexn_char!(str, 1);
    println!("{}", x);
}

#[test]
fn test_index_str() {
    let str = "abc";
    println!("{}", &str[1..2]);
}

#[test]
fn test_index_char() {
    let str = "abc";
    println!("{}", str.chars().nth(1).unwrap());
}