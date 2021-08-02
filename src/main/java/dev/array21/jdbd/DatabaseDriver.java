package dev.array21.jdbd;

import dev.array21.jdbd.datatypes.PreparedStatement;
import dev.array21.jdbd.datatypes.SqlRow;
import dev.array21.jdbd.exceptions.SqlException;

public interface DatabaseDriver {
	
	public SqlRow[] query(PreparedStatement statement) throws SqlException;
	public void execute(PreparedStatement statement) throws SqlException;
	public void unload();
}
