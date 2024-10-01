package net.winkassador.macbeth.util;

public class ArrayUtils {

    public static <T> T getValueOrDefault(T[] array, int index, T defaultValue) {
        if (index >= 0 && index < array.length) {
            return array[index];
        } else {
            return defaultValue;
        }
    }

}
