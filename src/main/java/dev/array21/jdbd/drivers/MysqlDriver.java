package dev.array21.jdbd.drivers;

import java.io.File;
import java.io.IOException;

import dev.array21.jdbd.DatabaseDriver;
import dev.array21.jdbd.PreparedStatement;
import dev.array21.jdbd.datatypes.SqlRow;
import dev.array21.jdbd.exceptions.DriverUnloadedException;
import dev.array21.jdbd.exceptions.UnboundPreparedStatementException;
import dev.array21.jdbd.exceptions.UnsupportedOperatingSystemException;
import dev.array21.jdbd.util.LibraryUtils;
import dev.array21.jdbd.util.Pair;

public class MysqlDriver implements DatabaseDriver {

	private static boolean LIBRARY_LOADED = false;

	private String host;
	private String username;
	private String password;
	private String database;
	
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
	 * @throws IOException When saving the native library failed
	 * @throws UnsatisfiedLinkError When loading the native library failed
	 * @throws UnsupportedOperatingSystemException When the current operating system is unsupported
	 */
	protected MysqlDriver(String host, String username, String password, String database) throws IOException {
		this.host = host;
		this.username = username;
		this.password = password;
		this.database = database;
		
		loadLibrary();
		
		char[] errorBuffer = new char[1024];
		this.ptr = initialize(errorBuffer);
		if(this.ptr == 0) {
			throw new RuntimeException("Failed to load driver: " + String.valueOf(errorBuffer));
		}
	}
	
	/**
	 * Load the native library
 	 * @throws IOException When saving the native library failed
	 * @throws UnsatisfiedLinkError When loading the native library failed
	 * @throws UnsupportedOperatingSystemException When the current operating system is unsupported
	 */
	private void loadLibrary() throws IOException {
		if(!LIBRARY_LOADED) {
			String path = LibraryUtils.getLibraryPath("libjdbd-mysql");
			Pair<File, File> filePair = LibraryUtils.saveLibrary(path);
			System.loadLibrary(filePair.getB().getAbsolutePath());
			
			LIBRARY_LOADED = true;
		}
	}
	
	/**
	 * Check if the current state of the driver is valid
	 * @throws IllegalStateException When the native library is not loaded
	 * @throws DriverUnloadedException When {@link #unload()} has already been called
	 */
	private void checkValid(){
		if(!LIBRARY_LOADED) {
			throw new IllegalStateException("libjdbd-mysql is not loaded");
		}
		
		if(!this.ptrValid) {
			throw new DriverUnloadedException("Driver has already been unloaded");
		}
	}
	
	/** Query the MySQL database
	 * @param statement The statement to query with
	 * @throws IllegalStateException When the native library is not loaded
	 * @throws DriverUnloadedException When {@link #unload()} has already been called
	 * @throws UnboundPreparedStatementException When not all parameters in the statement have been bound
	 */
	@Override
	public synchronized SqlRow[] query(PreparedStatement statement) {
		checkValid();
		
		if(!statement.allBound()) {
			throw new UnboundPreparedStatementException("Not all paramaters are bound");
		}
		
		return this.query(this.ptr, statement.getStmt());
	}

	/**
	 * Execute a {@link PreparedStatement}
	 * @param statement The statemenent to execute
	 * @throws IllegalStateException When the native library is not loaded
	 * @throws DriverUnloadedException When {@link #unload()} has already been called
	 * @throws UnboundPreparedStatementException When not all parameters in the statement have been bound
	 */
	@Override
	public synchronized void execute(PreparedStatement statement) {
		checkValid();
		
		if(!statement.allBound()) {
			throw new UnboundPreparedStatementException("Not all paramaters are bound");
		}
		
		this.execute(this.ptr, statement.getStmt());
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
		this.unload(this.ptr);
		this.ptr = 0;
	}

	/**
	 * Initialize the driver
	 * @param errorBuffer The buffer to put errors in
	 * @return Returns a heap pointer to where the mysql connection pool is stored
	 */
	private native long initialize(char[] errorBuffer);
	
	/**
	 * Execute a statement
	 * @param ptr The heap pointer to where the mysql connection pool is stored
	 * @param preparedStatement The statement to execute, with all params bound
	 * @return The amount of rows affected
	 */
	private native int execute(long ptr, String preparedStatement);
	
	/**
	 * Query the database
	 * @param ptr The heap pointer to where the mysql connection pool is stored
	 * @param preparedStatement The statement to query with, with all params bound
	 * @return The data returned by the database
	 */
	private native SqlRow[] query(long ptr, String preparedStatement);
	
	/**
	 * Unload the driver. This will destory the mysql connection pool and free it's memory
	 * @param ptr The heap pointer to where the mysql connection pool is stored
	 */
	private native void unload(long ptr);
}
