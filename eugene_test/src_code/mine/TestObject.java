public class TestObject {
//    public static void main(String[] args) throws InterruptedException {
//        int a = 1;
//        Thread thread = new Thread(
//                new Runnable() {
//                    @Override
//                    public void run() {
//                        int a = 1;
//                        while (a <= 5) {
//                            System.out.println(a++);
//                        }
//                    }
//                });
//
//        thread.start();
//        thread.join();
//
//        System.out.println("我是小老头赵帅  哈哈哈哈哈  没想到吧  我有出现了");
//        System.out.println(a);
//    }

    public static void main(String[] args) {
        Object a = new Object();
        String b = new String("dsadas");
        int c = 3;
        double[] d = new double[]{2, 3, 4};
        String[] e = new String[]{"a", "b", "c"};

        add(a, b, c, d, e);
    }

    public static void add(Object a, Object b, Object c, Object d, Object e) {
        System.out.println(a);
        System.out.println(b);
        System.out.println(c);
        System.out.println(d);
        System.out.println(e);
    }
}
