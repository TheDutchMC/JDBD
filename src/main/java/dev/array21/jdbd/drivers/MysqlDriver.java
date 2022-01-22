package dev.array21.jdbd.drivers;

import java.io.IOException;
import java.sql.SQLException;
import java.util.Arrays;

import dev.array21.jdbd.DatabaseDriver;
import dev.array21.jdbd.datatypes.PreparedStatement;
import dev.array21.jdbd.datatypes.SqlParameter;
import dev.array21.jdbd.datatypes.SqlRow;
import dev.array21.jdbd.exceptions.DriverUnloadedException;
import dev.array21.jdbd.exceptions.SqlException;
import dev.array21.jdbd.exceptions.UnboundPreparedStatementException;
import dev.array21.jdbd.exceptions.UnsupportedOperatingSystemException;

public class MysqlDriver implements DatabaseDriver {

	// DO NOT RENAME
	private String errorBuffer;
	private String host;
	private String username;
	private String password;
	private String database;
	// END
	
	/**
	 * Pointer to heap memory where the MySQL connection pool lives.
	 * 
	 * <h2> SAFETY </h2>
	 * This value should <strong>NEVER</strong> be altered, except by:
	 * <ul>
	 * 	<li> {@link MysqlDriver#MysqlDriver(String, String, String, String) }
	 * 	<li> {@link MysqlDriver#unload() }
	 * </ul>
	 * 
	 * Otherwhise altering this variable is <strong>guaranteed</strong> to result in Undefined Behaviour, and likely a segmentation fault
	 */
	private long ptr;
	private volatile boolean ptrValid = false;
	
	private MysqlDriver() {}
	
	/**
	 * Create a new MysqlDriver
	 * 
	 * <h2> Thread Safety </h2>
	 * It is up to the implementer to guarantee that this method is <strong>NOT</strong> called from two threads simultaneously, this can result in Undefined Behaviour
	 * @param host The MySQL host
	 * @param username The MySQL username
	 * @param password The MySQL password
	 * @param database The MySQL database
	 */
	protected MysqlDriver(String host, String username, String password, String database) {
		this.host = host;
		this.username = username;		
		this.password = password;
		this.database = database;
	}
	
	/**
	 * Load the native driver
	 * @throws IOException When saving the native library failed
	 * @throws UnsatisfiedLinkError When loading the native library failed
	 * @throws UnsupportedOperatingSystemException When the current operating system is unsupported
	 * @throws RuntimeException When {@link MysqlDriver#initializeNative()} returns an error
	 */
	public void loadDriver() throws IOException {
		DriverManager.loadLibrary();
		
		this.ptr = initializeNative();
		if(this.ptr == 0) {
			throw new RuntimeException("Failed to load driver: " + this.errorBuffer);
		}
		
		this.ptrValid = true;
	}
	
	/**
	 * Check if the current state of the driver is valid
	 * @throws IllegalStateException When the native library is not loaded
	 * @throws DriverUnloadedException When {@link #unload()} has already been called
	 */
	private void checkValid() {
		if(!DriverManager.isLoaded()) {
			throw new IllegalStateException("libjdbd is not loaded");
		}
		
		if(!this.ptrValid) {
			throw new DriverUnloadedException("Driver has already been unloaded");
		}
	}
	
	/**
	 * Check if the driver is loaded. When:
	 * <ul>
	 * 	<li> The library is loaded
	 * 	<li> The driver has been initialized
	 * </ul>
	 * @return True if it is loaded, false otherwhise
	 */
	public boolean isLoaded() {
		return DriverManager.isLoaded() && this.ptrValid;
	}
	
	/** Query the MySQL database
	 * @param statement The statement to query with
	 * @throws IllegalStateException When the native library is not loaded
	 * @throws DriverUnloadedException When {@link #unload()} has already been called
	 * @throws UnboundPreparedStatementException When not all parameters in the statement have been bound
	 * @throws SQLException
	 */
	@Override
	public synchronized SqlRow[] query(PreparedStatement statement) throws SqlException {
		checkValid();
		
		if(!statement.allBound()) {
			throw new UnboundPreparedStatementException("Not all paramaters are bound");
		}
		
		SqlRow[] resultSet = this.queryNative(this.ptr, statement.getStmt(), statement.getParameters());
		if(resultSet == null) {
			String buffer = this.errorBuffer;
			this.errorBuffer = "";
			
			throw new SqlException(buffer);
		}
		
		return resultSet;
	}

	/**
	 * Execute a {@link PreparedStatement}
	 * @param statement The statemenent to execute
	 * @throws IllegalStateException When the native library is not loaded
	 * @throws DriverUnloadedException When {@link #unload()} has already been called
	 * @throws UnboundPreparedStatementException When not all parameters in the statement have been bound
	 */
	@Override
	public synchronized void execute(PreparedStatement statement) throws SqlException {
		checkValid();
		
		if(!statement.allBound()) {
			throw new UnboundPreparedStatementException("Not all paramaters are bound");
		}

		int status = this.executeNative(this.ptr, statement.getStmt(), statement.getParameters());
		if(status != 0) {
			String buffer = this.errorBuffer;
			this.errorBuffer = "";
			
			throw new SqlException(buffer);
		}
	}
	
	/**
	 * Unload the driver
	 * @throws IllegalStateException When the native library is not loaded
	 * @throws DriverUnloadedException When {@link #unload()} has already been called
	 */
	@Override
	public synchronized void unload() {
		checkValid();
		this.ptrValid = false;
		this.unloadNative(this.ptr);
		this.ptr = 0;
	}

	/**
	 * Initialize the driver
	 * @return Returns a heap pointer to where the mysql connection pool is stored
	 */
	private synchronized native long initializeNative();
	
	/**
	 * Execute a statement
	 * @param ptr The heap pointer to where the mysql connection pool is stored
	 * @param rawStmt The statement to execute, with all params bound
	 * @return -1 if an error occurred. 0 if everything is OK.
	 */
	private synchronized native int executeNative(long ptr, String rawStmt, SqlParameter[] parameters);
	
	/**
	 * Query the database
	 * @param ptr The heap pointer to where the mysql connection pool is stored
	 * @param rawStmt The statement to query with, with all params bound
	 * @return The data returned by the database, or null if an error occurred
	 */
	private synchronized native SqlRow[] queryNative(long ptr, String rawStmt, SqlParameter[] parameters);
	
	/**
	 * Unload the driver. This will destory the mysql connection pool and free it's memory
	 * @param ptr The heap pointer to where the mysql connection pool is stored
	 */
	private synchronized native void unloadNative(long ptr);
}
