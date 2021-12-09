# Detail:
1. 为何链表转红黑树的时候长度选择8？  
   ![img.png](imgs/img.png)  
   从源码说明看链表长度符合柏松分布，如下所示: 显示链表长度k的概率分布
   链表长度达到8的概率已经非常低，综合考虑转换为红黑树的性能和链表查询性能，只有在极端情况下才进行转换，大部分情况下链表都不长。
2. 网络字节序，Java Class文件使用了网络字节序，为了在小端字节序intel x86和大端字节序risc之间维持平台独立性，必须保证固定的字节顺序。
   因此，jvm使用了用于网络传输的网络字节序，网络字节序属于大端。
   大小端字节序相关文章 https://www.ruanyifeng.com/blog/2016/11/byte-order.html
3. zero cost abstraction零开销抽象指的是你在构建一个抽象的时候，这个抽象不会造成额外的负担，典型的对比是struct和Java的class，如果java的类A有类B的成员，
   那么通过这个A类对象访问B成员事实上需要两次指针访问，但如果是rust的struct，你直接把它分配到栈上，那直接可以访问到了，
   虽然我们做出了抽象，但是并没有为抽象支付成本，和你不抽象直接把东西放一起是一样的。
4. Rust  usize是指该平台上理论上内存对象的最大大小，一般就是指指针能表示的地址空间，是一个与程序位数，平台都相关的类型，Rust中各种迭代都是推荐用usize的，比C++要统一
   官方直接要求使用usize，且不允许隐式转换。
   如果是u32在64位平台就没法覆盖全部寻址空间，u64在32位平台就需要将一条指令的事情转换为两条指令
5. rt.jar代表runtime JAR，并且包含引导类（bootstrap classes）——来自Core Java API的所有类。
6. [question mark operator and unwrap](https://m4rw3r.github.io/rust-questionmark-operator)
   1. ?操作符要求方法返回值要是Result才可以   unwrap()不需要对方法返回值做限定  遇到问题直接会调用panic!()进行抛出异常
7. The function syntax (Rc::clone(&rc)) makes it clear you're only making a new shared reference (cheap), 
   rather than cloning the underlying object being referenced (maybe expensive). 
   For arbitrary reference counted types, it may not be clear if a shallow or deep copy is occurring.
8. 对于method中max_locals的理解  
   首先获取CodeAttribute属性会出现两种情况 一种是方法有CodeAttribute属性，一种是没有  
   1. 当有CodeAttribute属性代表是非native、abstract、interface method，此时使用javac编译带好的max_locals作为method的max_locals就可以
   2. 当没有CodeAttribute属性时，代表是native、abstract、interface method中的一种，因为这些方法都是没有任何方法内容，因此此时其max_locals应该为0
9. 关于常量池的记录
   * ClassFile中的常量池会在classloader将其读入到方法区的时候，将String类型的常量放到string全局常量池
   * 常量池分为ClassFile中的ConstantPool和全局的String常量池
10. 用户自定义的ClassLoader存在的意义
   假设我们需要加载两个Object.class文件，默认是从启动类加载器中加载的，如果还想要加载另一个，因为启动类加载器的路径已经限定好了，因此此时需要一个新的类加载器
   用新的类加载器指定特定的路径，然后用其再加载Object.class文件，就可以将同名的class文件加载进内存    
   [good blog](https://blog.csdn.net/u011212394/article/details/104113847)
11. 默认卸载class的条件
    * 该类所有的实例都已经被回收
    * 加载该类的ClassLoader已经被回收
    * 该类对应的Class对象没有在任何地方被引用，无法在任何地方通过反射访问该类的方法
   如果以上三个条件全部满足，jvm就会在方法区垃圾回收的时候对类进行卸载，类的卸载过程就是在方法区中清空类信息，java类的整个生命周期就结束了。  
    [good blog](https://blog.csdn.net/xihuanyuye/article/details/89887913)
12. <strong>思考以下情景：</strong>
    1. 首先，是为了区分同名的类：假定存在一个应用服务器，上面部署着许多独立的应用，同时他们拥有许多同名却不同版本的类库。
       试想，这时候 jvm 该怎么加载这些类同时能尽可能地避免掉类加载时对同名类的差异检测呢？当然是不同的应用都拥有自己独立的类加载器了。
    2. 其次，是为了更方便地加强类的能力：类加载器可以在 load class 时对 class 进行重写和覆盖，在此期间就可以对类进行功能性的增强。
       比如添加面向切面编程时用到的动态代理，以及 debug 等原理。怎么样达到仅修改一个类库而不对其他类库产生影响的效果呢？一个比较方便的模式就是每个类库都可以使用独立的类加载器  
   <strong>小结：</strong>  
   jvm 需要有不同的类加载器，因为它一方面允许你在一个 jvm 里运行不同的应用程序，另一方面方便你独立的对不同类库进行运行时增强。  
   [good blog](https://segmentfault.com/q/1010000014745626)
13. 
   



## Not Important:
1. 当filter条件为false过滤此元素，而true则保留此元素。
2. 