package dev.array21.jdbd.exceptions;

public class UnsupportedOperatingSystemException extends RuntimeException{
	private static final long serialVersionUID = 1708054966763615076L;

	public UnsupportedOperatingSystemException() {
		super();
	}
	
	public UnsupportedOperatingSystemException(String msg) {
		super(msg);
	}
	
}
