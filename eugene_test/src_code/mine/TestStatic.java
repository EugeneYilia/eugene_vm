public class TestStatic {
    public static void main(String[] args) {
        Children children = new Children();
        System.out.println(Children.number);
        System.out.println(Children.number2);
        Children.number = 4;
        a(Children.number);
        System.out.println(Children.number);
        System.out.println(Children.number2);
    }

    public static void a(int number){
        number = 3;
    }
}

class Parent {
    static int number = 1;
}

class Children extends Parent{
    static int number2 = 2;
}