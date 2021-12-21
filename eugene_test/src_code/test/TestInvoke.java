public class TestInvoke {
    void a(){
        int x = 1;
        b();
        int y = 3;
    }

    void b (){
        int y = 2;
    }

    void c(int a,int b){
        System.out.println(a);
        System.out.println(b);
    }

    void d(){
        int a = 2;
        int b = 3;
        c(a,b);
    }

    void c(){
        int c= 9;
        System.out.println(c);
    }
}
