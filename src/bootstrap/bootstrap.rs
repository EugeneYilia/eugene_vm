use crate::bootstrap::bootstrap_option::BootstrapOption;

// class_name 主函数入口
// user_classpath and boot_classpath需要先解析出来
pub fn start_jvm(bootstrap_option:BootstrapOption){
    let classpath = parse_classpath(bootstrap_option.boot_classpath_option,bootstrap_option.user_classpath_option);

}