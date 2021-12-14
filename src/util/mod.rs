pub mod class_util;
pub mod converter;
pub mod file_util;


#[cfg(test)]
mod tests {
    use std::num::Wrapping;

    #[test]
    fn test_wrapping() {
        let zero = Wrapping(0u8);
        let one = Wrapping(1u8);
        let result = zero - one;
        println!("{}", result);
    }

    #[test]
    fn test_add() {
        let _zero = 0u8;
        let _one = 1u8;
        // let result = _zero - _one;
        // println!("{}", result);
    }
}