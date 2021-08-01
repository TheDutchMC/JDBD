package dev.array21.jdbd;

import java.util.ArrayList;
import java.util.List;

public class PreparedStatement {
	private String stmt;
	private int[] bindingOffsets;
	private int lenDelta;
	
	public PreparedStatement(String stmt) {
		this.stmt = stmt;
		
		List<Integer> bindingOffsets = new ArrayList<>();
		char[] stmtChars = stmt.toCharArray();
		for(int i = 0; i < stmtChars.length; i++) {
			char c = stmtChars[i];
			if(c == '?') {
				bindingOffsets.add(i);
			}
		}
		
		this.bindingOffsets = bindingOffsets.stream().mapToInt(i->i).toArray();
	}
	
	public String getStmt() {
		return this.stmt;
	}
	
	public PreparedStatement bind(int pos, String str) {
		int bindOffset = this.bindingOffsets[pos] + this.lenDelta;
		this.lenDelta += str.length() -1;
		
		String beforeBind = this.stmt.substring(0, bindOffset);
		String afterBind = this.stmt.substring(bindOffset + 1, this.stmt.length());
		
		this.stmt = beforeBind + str + afterBind;
		return this;
	}
	
	public PreparedStatement bind(int pos, int i) {
		return this.bind(pos, String.valueOf(i));
	}
	
	public PreparedStatement bind(int pos, boolean b) {
		return this.bind(pos, String.valueOf(b));
	}
}
