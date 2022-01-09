
/**
 * @author EugeneLiu
 * @date 2021-12-13
 */

public class TestAdd {
    String author = "EugeneLiu";

    public static void main(String[] args) {
        int a = 0;
        int b = 1;
        int sum = a + b;
        System.out.println(sum);
        System.out.println(sum);
    }

    public int a(int b, int c) {
        return b + c;
    }

    // 局部变量表1 77 q 方法入参第一个
    // 局部变量表2 88 w 方法入参第二个
    // stack bottom - [this, 77, 88] - stack head
    public void x() {
        int q = 77;
        int w = 88;
        a(q, w);
    }
}
// Concise
// public class TestAdd {
//  public TestAdd();
//    Code:
//       0: aload_0
//       1: invokespecial #1                  // Method java/lang/Object."<init>":()V
//       4: return
//
//  public static void main(java.lang.String[]);
//    Code:
//       0: iconst_0
//       1: istore_1
//       2: iconst_1
//       3: istore_2
//       4: iload_1
//       5: iload_2
//       6: iadd
//       7: istore_3
//       8: getstatic     #2                  // Field java/lang/System.out:Ljava/io/PrintStream;
//      11: iload_3
//      12: invokevirtual #3                  // Method java/io/PrintStream.println:(I)V
//      15: return
// }

// Detail:
// // class version 52.0 (52)
// // access flags 0x21
//    public class TestAdd {
//
//      // compiled from: TestAdd.java
//
//      // access flags 0x1
//      public <init>()V
//       L0
//        LINENUMBER 1 L0
//        ALOAD 0
//        INVOKESPECIAL java/lang/Object.<init> ()V
//        RETURN
//        MAXSTACK = 1
//        MAXLOCALS = 1
//
//      // access flags 0x9
//      public static main([Ljava/lang/String;)V
//       L0
//        LINENUMBER 3 L0
//        ICONST_0
//        ISTORE 1
//       L1
//        LINENUMBER 4 L1
//        ICONST_1
//        ISTORE 2
//       L2
//        LINENUMBER 5 L2
//        ILOAD 1
//        ILOAD 2
//        IADD
//        ISTORE 3
//       L3
//        LINENUMBER 6 L3
//        GETSTATIC java/lang/System.out : Ljava/io/PrintStream;
//        ILOAD 3
//        INVOKEVIRTUAL java/io/PrintStream.println (I)V
//       L4
//        LINENUMBER 7 L4
//        RETURN
//        MAXSTACK = 2
//        MAXLOCALS = 4
//    }
//