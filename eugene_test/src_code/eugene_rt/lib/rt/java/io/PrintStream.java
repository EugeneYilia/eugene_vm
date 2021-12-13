package java.io;

import java.eugene.EugeneStream;

/**
 * @author EugeneLiu
 * @date 2021-12-13
 */

public class PrintStream {
    private final EugeneStream eugeneStream;
    public PrintStream(EugeneStream eugeneStream){
        this.eugeneStream = eugeneStream;
    }

    public void println(String content){
        eugeneStream.println(content);
    }

    public void println(int content){
        eugeneStream.println(content);
    }
}
