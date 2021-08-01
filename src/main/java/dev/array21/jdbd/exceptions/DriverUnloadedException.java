package dev.array21.jdbd.exceptions;

public class DriverUnloadedException extends RuntimeException {
	private static final long serialVersionUID = -1950674775003130668L;

	public DriverUnloadedException() {
		super();
	}
	
	public DriverUnloadedException(String msg) {
		super(msg);
	}
}
