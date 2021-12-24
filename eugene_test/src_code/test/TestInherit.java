//import java.util.HashMap;
//import java.util.Objects;
//
//public class TestInherit {
//    public static void main(String[] args) {
////        B b = new B();
////        System.out.println(b.author);
//    }
//
//    public void a(){
//        int a = 888888;
//        long b = 666666;
////        long c = a + b;
////        System.out.println(a + 2);
//    }
//}
//
//class A {
//    protected String author = "A";
//}
//
//class B extends A {
////    public Integer author = 33;
//}
//
//class Data {
//    int accessFlags;
//    String name;
//    String descriptor;
//
//    public Data(int accessFlags, String name, String descriptor) {
//        this.accessFlags = accessFlags;
//        this.name = name;
//        this.descriptor = descriptor;
//    }
//
//    @Override
//    public boolean equals(Object o) {
//        if (this == o) return true;
//        if (!(o instanceof Data)) return false;
//        Data data = (Data) o;
//        return Objects.equals(name, data.name);
//    }
//
//    @Override
//    public int hashCode() {
//        return Objects.hash(name);
//    }
//
//    public static void main(String[] args) {
//        HashMap<Data, Integer> map = new HashMap<>();
//        map.put(new Data(1, "1", "1"), 1);
//        map.put(new Data(2, "1", "2"), 2);
//        map.put(new Data(3, "1", "3"), 3);
//        map.forEach((key,value) -> System.out.println(key+" : "+value));
//    }
//
//    @Override
//    public String toString() {
//        return "Data{" +
//                "accessFlags=" + accessFlags +
//                ", name='" + name + '\'' +
//                ", descriptor='" + descriptor + '\'' +
//                '}';
//    }
//}
//
//class XA {
//    String name = "X";
//}
//
//class XB extends XA{
//    String name = "c";
//
//    public static void main(String[] args) {
//        XB b = new XB();
//        System.out.println(b.name);
//        System.out.println(b.getClass());
//    }
//}