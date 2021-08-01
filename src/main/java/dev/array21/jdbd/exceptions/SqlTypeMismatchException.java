package dev.array21.jdbd.exceptions;

public class SqlTypeMismatchException extends RuntimeException {
	private static final long serialVersionUID = 2333815588765994170L;

	public SqlTypeMismatchException() {
		super();
	}
	
	public SqlTypeMismatchException(String msg) {
		super(msg);
	}
}
