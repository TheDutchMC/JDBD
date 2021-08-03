package dev.array21.jdbd.drivers;

import java.io.IOException;

import dev.array21.jdbd.exceptions.UnsupportedOperatingSystemException;

public class PostgreSqlDriverFactory {
	private String host;
	private String username;
	private String password;
	private String database;
	
	/**
	 * Set the PostgreSQL host
	 * @param host The host to set
	 * @return The current factory
	 */
	public PostgreSqlDriverFactory setHost(String host) {
		this.host = host;
		return this;
	}
	
	/**
	 * Set the PostgreSQL username
	 * @param username The username to set
	 * @return The current factory
	 */
	public PostgreSqlDriverFactory setUsername(String username) {
		this.username = username;
		return this;
	}
	
	/**
	 * Set the PostgreSQL password
	 * @param password The password to set
	 * @return The current factory
	 */
	public PostgreSqlDriverFactory setPassword(String password) {
		this.password = password;
		return this;
	}
	
	/**
	 * Set the PostgreSQL database
	 * @param database The database to set
	 * @return The current factory
	 */
	public PostgreSqlDriverFactory setDatabase(String database) {
		this.database = database;
		return this;
	}
	
	/**
	 * Build the PostgreSqlDriver. Throws an IllegalStateException when:
	 * <ul>
	 * 	<li> The host is unsert
	 * 	<li> The database is unset
	 * </ul>
	 * 
	 * <h2> Thread Safety </h2>
	 * It is up to the implementer to guarantee that this method is <strong>NOT</strong> called from two threads simultaneously, this can result in Undefined Behaviour
	 * @return Return an instance of the PostgreSqlDriver
	 * @throws IOException When saving the native library failed
	 * @throws UnsatisfiedLinkError When loading the native library failed
	 * @throws UnsupportedOperatingSystemException When the current operating system is unsupported
	 */
	public PostgreSqlDriver build() throws IOException {
		if(this.host == null) {
			throw new IllegalStateException("Host is unset");
		}
		
		if(this.database == null) {
			throw new IllegalStateException("Database is unset");
		}
		
		PostgreSqlDriver driver = new PostgreSqlDriver(this.host, this.username, this.password, this.database);
		driver.loadDriver();
		return driver;
	}
}
