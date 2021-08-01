package dev.array21.jdbd.drivers;

public class MysqlDriverFactory {

	private String host;
	private String username;
	private String password;
	private String database;
	
	public MysqlDriverFactory setHost(String host) {
		this.host = host;
		return this;
	}
	
	public MysqlDriverFactory setUsername(String username) {
		this.username = username;
		return this;
	}
	
	public MysqlDriverFactory setPassword(String password) {
		this.password = password;
		return this;
	}
	
	public MysqlDriverFactory setDatabase(String database) {
		this.database = database;
		return this;
	}
	
	public MysqlDriver build() {
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
