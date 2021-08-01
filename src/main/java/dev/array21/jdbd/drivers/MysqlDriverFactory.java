package dev.array21.jdbd.drivers;

import java.io.IOException;

import dev.array21.jdbd.exceptions.UnsupportedOperatingSystemException;

public class MysqlDriverFactory {

	private String host;
	private String username;
	private String password;
	private String database;
	
	/**
	 * Set the MySQL host
	 * @param host The host to set
	 * @return The current factory
	 */
	public MysqlDriverFactory setHost(String host) {
		this.host = host;
		return this;
	}
	
	/**
	 * Set the MySQL username
	 * @param username The username to set
	 * @return The current factory
	 */
	public MysqlDriverFactory setUsername(String username) {
		this.username = username;
		return this;
	}
	
	/**
	 * Set the MySQL password
	 * @param password The password to set
	 * @return The current factory
	 */
	public MysqlDriverFactory setPassword(String password) {
		this.password = password;
		return this;
	}
	
	/**
	 * Set the MySQL database
	 * @param database The database to set
	 * @return The current factory
	 */
	public MysqlDriverFactory setDatabase(String database) {
		this.database = database;
		return this;
	}
	
	/**
	 * Build the MsqlDriver. Throws an IllegalStateException when:
	 * <ul>
	 * 	<li> The host is unsert
	 * 	<li> The database is unset
	 * </ul>
	 * 
	 * <h2> Thread Safety </h2>
	 * It is up to the implementer to guarantee that this method is <strong>NOT</strong> called from two threads simultaneously, this can result in Undefined Behaviour
	 * @return Return an instance of the MysqlDriver
	 * @throws IOException When saving the native library failed
	 * @throws UnsatisfiedLinkError When loading the native library failed
	 * @throws UnsupportedOperatingSystemException When the current operating system is unsupported
	 */
	public MysqlDriver build() throws IOException {
		if(this.host == null) {
			throw new IllegalStateException("Host is unset");
		}
		
		if(this.database == null) {
			throw new IllegalStateException("Database is unset");
		}
		
		MysqlDriver driver = new MysqlDriver(this.host, this.username, this.password, this.database);
		return driver;
	}
	
}
