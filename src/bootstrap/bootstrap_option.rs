pub struct BootstrapOption {
    // 启动函数入口
    pub class_name: String,
    // 自己需要额外增加的classpath路径
    pub classpath_option: Option<String>,
    // java runtime environment需要增加的参数
    pub jre_option: Option<String>,
    // 额外传入的启动参数
    pub args: Vec<String>,
}