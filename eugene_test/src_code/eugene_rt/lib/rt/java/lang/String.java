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

    /**
     * JDK source code      char => byte[]
     * private void put(char c, ByteBuffer dst) {
     * if (byteOrder == BIG) {
     * dst.put((byte)(c >> 8));
     * dst.put((byte)(c & 0xff));
     * } else {
     * dst.put((byte)(c & 0xff));
     * dst.put((byte)(c >> 8));
     * }
     * }
     */
    public byte[] getBytes() {
        // only support UTF-16 now
        byte[] bytes = new byte[2 * this.value.length + 2];
        bytes[0] = -2;
        bytes[1] = -1;
        for (int i = 0; i < value.length; i++) {
            bytes[i * 2 + 2] = (byte) ((value[i] & 0xff00) >>> 8);
            bytes[i * 2 + 1 + 2] = (byte) (value[i] & 0x00ff);
        }
        return bytes;
    }
}

