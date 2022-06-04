public class TestStatic {
    public static void main(String[] args) throws InterruptedException {
        Children children = new Children();
        System.out.println(Children.number);
        System.out.println(Children.number2);
        Children.number = 4;
        a(Children.number);
        System.out.println(Children.number);
        System.out.println(Children.number2);

        System.out.println("XXXXXXXXXXXXXX");
        System.out.println(xxx() + 333.3888);
        int a = 4;
        int[][] x = new int[a][a];
        while(true){

        }
    }

    public static  float xxx() {
        return Float.NaN;
    }

    public static void a(int number) {
        number = 3;
    }
}

class Parent {
    static int number = 1;
}

class Children extends Parent {
    static int number2 = 2;
}