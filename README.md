# eugene_vm

## Module Description

1. ClassLoader & ClassReader
    * ClassReader负责字节的读取与转换，构造出来ClassFile
    * ClassLoader
2. Runtime - method_area
   * Slot: 局部变量表中最基本的存储单元，称之为变量槽，32位以内的数据占用一个Slot，64位类型的占用两个Slot
   * 局部变量表中存储的是局部变量，包含8种基本数据类型，引用数据类型和returnAddress类型的数据

## TODO

1. 字符串常量池是共有的
2. ldc string的时候如何将ref更好的推到operand stack
3. class_loader加入双亲委派机制
7. 增加更多的字节码指令
8. 多线程的支持
9. String的实现 a. class_loader obj_descriptor
10. 内置函数的实现

## 重构方案

## 过程记录

1. 为什么字节码指令集处理中，传递的入参是thread而不是stack_frame?  
   这是因为在字节码执行的过程中，需要借用thread，之后在xreturn方法内需要获取到thread之后将stack_frame给移除top的一个 这样就会产生需要借用borrow_mut两次thread的情况 这样是有问题的
   不如在第一次借用到thread之后就一直传递借用到的thread 之后也用这个进行处理 如果传递的参数是thread和stack_frame 就会很冗余 因此综合来看 传递一个thread到字节码指令的入参比较合适
2.

## Good blog

### Write OS with Rust

1. https://blog.csdn.net/qq_41698827/article/details/103393993?spm=1001.2101.3001.6661.1&utm_medium=distribute.pc_relevant_t0.none-task-blog-2%7Edefault%7ECTRLIST%7Edefault-1.no_search_link&depth_1-utm_source=distribute.pc_relevant_t0.none-task-blog-2%7Edefault%7ECTRLIST%7Edefault-1.no_search_link&utm_relevant_index=1
2. https://blog.csdn.net/qq_41698827/category_9553802.html
3. https://blog.csdn.net/qq_41698827/article/details/104014935
4. https://baike.baidu.com/item/8259A/11048399
5. https://blog.csdn.net/weixin_46716100/article/details/122205489
6. https://www.cnblogs.com/vinozly/p/6102804.html
7. https://blog.csdn.net/qq_43546328/article/details/109680374
8. https://www.zhihu.com/question/271600057
9. https://cn.bing.com/search?q=cpu%E7%9A%84%E4%B8%AD%E6%96%AD%E6%98%AF%E4%BB%80%E4%B9%88&cvid=0d372706c5ad43e7b08ada280bd0f8e9&aqs=edge..69i57.6402j0j1&pglt=2083&FORM=ANNTA1&PC=DCTS