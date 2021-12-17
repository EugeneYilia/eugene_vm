import java.nio.charset.StandardCharsets;
import java.util.Arrays;

/**
 * @author EugeneLiu
 * @date 2021-12-17
 */

public class TestString {
    public static void main(String[] args) {
        // [-2, -1, 0, 69, 0, 117, 0, 103, 0, 101, 0, 110, 0, 101, 0, 76, 0, 105, 0, 117]
        String author = "EugeneLiu";
        System.out.println(Arrays.toString(author.getBytes(StandardCharsets.UTF_16)));

        System.out.println(Arrays.toString(getBytes(author.toCharArray())));

        byte[] strangeArray = new byte[]{-2,-1};
//        strangeArray.
        String newString = new String(strangeArray, StandardCharsets.UTF_16);
        System.out.println(newString);
    }

    public static byte[] getBytes(char[] value) {
        // only support UTF-16 Big-Endian now
        byte[] bytes = new byte[2 * value.length + 2];
        bytes[0] = -2;
        bytes[1] = -1;
        for (int i = 0; i < value.length; i++) {
            bytes[i * 2 + 2] = (byte) (value[i] >> 8);
            bytes[i * 2 + 1 + 2] = (byte) (value[i] & 0xff);
        }
        return bytes;
    }
}
