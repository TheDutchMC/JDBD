package dev.array21.jdbd;

import dev.array21.jdbd.datatypes.SqlRow;

public interface DatabaseDriver {
	
	public SqlRow[] query(PreparedStatement statement);
	public void execute(PreparedStatement statement);
	public void unload();
}
