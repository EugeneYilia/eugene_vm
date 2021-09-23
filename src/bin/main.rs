use EugeneVM::bootstrap::bootstrap_option::BootstrapOption;
use EugeneVM::bootstrap::bootstrap::start_jvm;

fn main() {
    let source_class_name = "";
    let class_name = source_class_name.replace(".","/");
    let bootstrap_option = BootstrapOption{
        class_name,
        classpath_option:None,
        jre_option:None,
        args:vec![]
    };
    start_jvm(bootstrap_option);
}
