package dev.array21.jdbd.drivers;

import java.io.File;
import java.io.IOException;

import dev.array21.jdbd.DatabaseDriver;
import dev.array21.jdbd.PreparedStatement;
import dev.array21.jdbd.datatypes.SqlRow;
import dev.array21.jdbd.exceptions.DriverUnloadedException;
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
	
	protected MysqlDriver(String host, String username, String password, String database) throws IOException, UnsatisfiedLinkError, UnsupportedOperatingSystemException {
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
	
	private void loadLibrary() throws IOException, UnsatisfiedLinkError, UnsupportedOperatingSystemException {
		if(!LIBRARY_LOADED) {
			String path = LibraryUtils.getLibraryPath("libjdbd-mysql");
			Pair<File, File> filePair = LibraryUtils.saveLibrary(path);
			System.loadLibrary(filePair.getB().getAbsolutePath());
			
			LIBRARY_LOADED = true;
		}
	}
	
	private void checkValid() throws DriverUnloadedException, IllegalStateException {
		if(!LIBRARY_LOADED) {
			throw new IllegalStateException("libjdbd-mysql is not loaded");
		}
		
		if(!this.ptrValid) {
			throw new DriverUnloadedException("Driver has already been unloaded");
		}
	}
	
	
	@Override
	public synchronized SqlRow[] query(PreparedStatement statement) throws DriverUnloadedException {
		checkValid();
		return this.query(this.ptr, statement.getStmt());
	}

	@Override
	public synchronized void execute(PreparedStatement statement) throws DriverUnloadedException {
		checkValid();
		this.execute(this.ptr, statement.getStmt());
	}
	
	@Override
	public synchronized void unload() throws DriverUnloadedException {
		checkValid();
		this.ptrValid = false;
		this.unload(this.ptr);
	}

	private native long initialize(char[] errorBuffer);
	private native int execute(long ptr, String preparedStatement);
	private native SqlRow[] query(long ptr, String preparedStatement);
	private native void unload(long ptr);
}
