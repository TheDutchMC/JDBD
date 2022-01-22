package dev.array21.jdbd.datatypes;

@SuppressWarnings("unused") // Accessed from JNI
public class SqlParameter {
    // All values are read from JNI and should not be renamed
    private final SqlParameterType type;
    private byte[] bytesVal;
    private long longVal;
    private float floatVal;
    private double doubleVal;

    public SqlParameter() {
        this.type = SqlParameterType.NULL;
    }

    public SqlParameter(byte[] bytes) {
        this.bytesVal = bytes;
        this.type = SqlParameterType.BYTES;
    }

    public SqlParameter(long longVal) {
        this.longVal = longVal;
        this.type = SqlParameterType.INT;
    }

    public SqlParameter(float floatVal) {
        this.floatVal = floatVal;
        this.type = SqlParameterType.FLOAT;
    }

    public SqlParameter(double doubleVal) {
        this.doubleVal = doubleVal;
        this.type = SqlParameterType.DOUBLE;
    }
}
