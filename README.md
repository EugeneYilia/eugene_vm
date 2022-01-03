# eugene_vm

## Module Description

1. ClassLoader & ClassReader
    * ClassReader负责字节的读取与转换，构造出来ClassFile
    * ClassLoader
2. Runtime - method_area
    * Slot: 局部变量表中最基本的存储单元，称之为变量槽，32位以内的数据占用一个Slot，64位类型的占用两个Slot
    * 局部变量表中存储的事局部变量，包含8种基本数据类型，引用数据类型和returnAddress类型的数据

## TODO

1. 字符串常量池是共有的
2. ldc string的时候如何将ref更好的推到operand stack
3. class_loader加入双亲委派机制
4. descriptor的校验
5. constant_value_index的校验
6. 执行<clinit>、<init>
7. 增加更多的字节码指令
8. 多线程的支持

## 重构方案

## 过程记录

1. 为什么字节码指令集处理中，传递的入参是thread而不是stack_frame?  
   这是因为在字节码执行的过程中，需要借用thread，之后在xreturn方法内需要获取到thread之后将stack_frame给移除top的一个 这样就会产生需要借用borrow_mut两次thread的情况 这样是有问题的
   不如在第一次借用到thread之后就一直传递借用到的thread 之后也用这个进行处理 如果传递的参数是thread和stack_frame 就会很冗余 因此综合来看 传递一个thread到字节码指令的入参比较合适
2. 