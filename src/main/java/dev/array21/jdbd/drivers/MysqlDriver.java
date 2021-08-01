package dev.array21.jdbd.drivers;

import dev.array21.jdbd.DatabaseDriver;
import dev.array21.jdbd.PreparedStatement;
import dev.array21.jdbd.datatypes.SqlRow;

public class MysqlDriver implements DatabaseDriver {

	private String host;
	private String username;
	private String password;
	private String database;
	
	private long ptr;
	private volatile boolean loaded = false;
	
	private MysqlDriver() {}
	
	protected MysqlDriver(String host, String username, String password, String database) {
		this.host = host;
		this.username = username;
		this.password = password;
		this.database = database;
		
		char[] errorBuffer = new char[1024];
		
		this.ptr = initialize(errorBuffer);
		if(this.ptr == 0) {
			throw new RuntimeException("Failed to load driver: " + String.valueOf(errorBuffer));
		}
	}
	
	@Override
	public synchronized SqlRow[] query(PreparedStatement statement) {
		// TODO Auto-generated method stub
		return null;
	}

	@Override
	public synchronized void execute(PreparedStatement statement) {
		// TODO Auto-generated method stub	
	}
	
	@Override
	public synchronized void unload() {
		this.loaded = false;
		this.unload();
	}

	private native long initialize(char[] errorBuffer);
	private native int execute(long ptr, String preparedStatement);
	private native SqlRow[] query(long ptr, String preparedStatement);
	private native void unload(long ptr);
}
