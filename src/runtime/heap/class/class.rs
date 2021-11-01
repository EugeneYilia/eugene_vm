
//Class文件格式如下：
//
// 类型	描述	备注
// u4	magic	魔数：0xCAFEBABE
// u2	minor_version	小版本号
// u2	major_version	主版本号
// u2	constant_pool_count	常量池大小，从1开始
// cp_info	constant_pool[constant_pool_count - 1]	常量池信息
// u2	access_flags	访问标志
// u2	this_class	类索引
// u2	super_class	父类索引
// u2	interfaces_count	接口个数
// u2	interfaces[interfaces_count]	接口类索引信息
// u2	fields_count	字段数
// field_info	fields[fields_count]	字段表信息
// u2	methods_count	方法数
// method_info	methods[methods_count]	方法表信息
// u2	attributes_count	属性个数
// attribute_info	attributes[attributes_count]	属性表信息
//
// 链接：https://www.jianshu.com/p/ae3f860499aa


use crate::runtime::method_area::constant_pool::constant_pool::ConstantPool;

#[derive(Debug)]
pub struct Class {
    pub access_flags : u16,

    // TODO： 考虑是否换成u16  对应于常量池中的常量
    pub name: String,

    pub constant_pool: ConstantPool,


}