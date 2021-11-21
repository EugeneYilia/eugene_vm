/***
    所有的access_flag 该method都要满足
 */
pub fn check_access_flags_all(method_access_flag: u16, access_flags: &Vec<u16>) -> bool {
    let find_result = access_flags.iter().find(|access_flag| {
        **access_flag & method_access_flag == 0
    });
    match find_result {
        Some(_) => false,
        None => true
    }
}