package java.lang;

import java.eugene.EugeneStream;
import java.io.PrintStream;

/**
 * @author EugeneLiu
 * @date 2021-12-13
 */

public class System {
    public static PrintStream out = new PrintStream(new EugeneStream());
    public static final String author = "EugeneLiu";
    public static final Object obj = new Object();
}
