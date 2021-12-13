package java.eugene;

import java.io.IOException;
import java.io.OutputStream;

/**
 * @author EugeneLiu
 * @date 2021-12-13
 */

public class EugeneStream extends OutputStream {
    public EugeneStream(){

    }

    @Override
    public native void write(int b) throws IOException;

    public native void println(String content);

    public native void println(int content);
}
