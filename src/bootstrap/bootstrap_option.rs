pub struct BootstrapOption {
    // 启动函数入口
    pub class_name: String,
    // 自己需要额外增加的classpath路径
    pub user_classpath_option: Option<String>,
    // 系统classpath路径
    pub boot_classpath_option: Option<String>,
    // 额外传入的启动参数
    pub args: Vec<String>,
}

impl BootstrapOption {
    pub fn new(class_name: &str, user_classpath_option: Option<String>, boot_classpath_option: Option<String>, args: Vec<String>) -> BootstrapOption {
        let class_name = class_name.replace(".", "/");
        BootstrapOption {
            class_name,
            user_classpath_option,
            boot_classpath_option,
            args,
        }
    }
}