package java.lang;

/**
 * @author EugeneLiu
 * @date 2021-12-16
 */

public final class String {
    private final char[] value;

    public String(char[] value) {
        this.value = new char[value.length];
        System.arraycopy(value, 0, this.value, 0, value.length);
    }

    public byte[] getBytes(){
        // only support UTF-16 now
        byte[] bytes = new byte[2 * this.value.length];

        return bytes;
    }
}
