# eugene_vm

## Module Description

1. ClassLoader & ClassReader
    * ClassReader负责字节的读取与转换，构造出来ClassFile
    * ClassLoader
2. Runtime - method_area
    * Slot: 局部变量表中最基本的存储单元，称之为变量槽，32位以内的数据占用一个Slot，64位类型的占用两个Slot
    * 局部变量表中存储的事局部变量，包含8种基本数据类型，引用数据类型和returnAddress类型的数据
    * 