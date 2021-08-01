package dev.array21.jdbd.exceptions;

public class UnboundPreparedStatementException extends RuntimeException {
	private static final long serialVersionUID = 6174140404480172178L;

	public UnboundPreparedStatementException() {
		super();
	}
	
	public UnboundPreparedStatementException(String msg) {
		super(msg);
	}
}
