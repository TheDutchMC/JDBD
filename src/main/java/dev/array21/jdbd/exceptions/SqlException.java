package dev.array21.jdbd.exceptions;

public class SqlException extends Exception {
	private static final long serialVersionUID = -5682698385934512793L;

	public SqlException() {
		super();
	}
	
	public SqlException(String msg) {
		super(msg);
	}
	
}
