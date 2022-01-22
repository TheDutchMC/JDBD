package dev.array21.jdbd.datatypes;

import java.nio.charset.StandardCharsets;

public class PreparedStatement {
	private String stmt;
	private SqlParameter[] parameters;

	public PreparedStatement(String stmt) {
		this.stmt = stmt;
		this.parameters = new SqlParameter[(int) stmt.chars().filter(x -> x == '?').count()];
	}

	public void bind(int pos, String val) {
		parameters[pos] = new SqlParameter(val.getBytes(StandardCharsets.UTF_8));
	}

	public void bind(int pos, byte[] val) {
		parameters[pos] = new SqlParameter(val);
	}

	public void bind(int pos, int val) {
		parameters[pos] = new SqlParameter((long) val);
	}

	public void bind(int pos, float val) {
		parameters[pos] = new SqlParameter(val);
	}

	public void bind(int pos, double val) {
		parameters[pos] = new SqlParameter(val);
	}

	public void bind(int pos, boolean val) {
		parameters[pos] = new SqlParameter(val ? 1L : 0L);
	}

	public String getStmt() {
		return this.stmt;
	}

	public SqlParameter[] getParameters() {
		return this.parameters;
	}

	public boolean allBound() {
		for(SqlParameter x : this.parameters) {
			if(x == null) {
				return false;
			}
		}

		return true;
	}
}
