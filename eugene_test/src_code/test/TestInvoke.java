public class TestInvoke {
    void a(){
        int x = 1;
        b();
        long m =4;
        int y = 3;
        int w = 8;
    }

    void b (){
        int y = 2;
    }

    void c(int a,long d,String e,String[] f,int b){
        System.out.println(a);
        System.out.println(b);
        System.out.println(e);
        System.out.println(d);
        System.out.println(f);
    }

    void d(){
        int a = 2;
        int b = 3;
        c(a,3,"a",new String[]{"a"},b);
    }

    void c(){
        int c= 9;
        System.out.println(c);
    }

    String e(){
        return "a";
    }
}
