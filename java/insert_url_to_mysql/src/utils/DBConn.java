package utils;

import java.sql.Connection;
import java.sql.DriverManager;
import java.sql.SQLException;

public class DBConn {
	
	private static final String driver = "com.mysql.jdbc.Driver";
	private static final String url = "jdbc:mysql://localhost:3306/website?useUnicode=true&characterEncoding=UTF-8";
	private static final String username = "root";
	private static final String password = "xingkong";
	
	//连接对象
	private static Connection conn = null;
	
	//静态代码块加载驱动
	static {
		try {
			Class.forName(driver);
		} catch (ClassNotFoundException e) {
			e.printStackTrace();
		}
	}
	
	//获得数据库连接
	public static Connection getConnection() {
		if(conn == null) {
			try {
				conn = DriverManager.getConnection(url, username, password);
				return conn;
			} catch (SQLException e) {
				e.printStackTrace();
			}
		}
		return conn;
	}
	
	public static void main(String[] args) {
		Connection conn = getConnection();
		if(conn != null) {
			System.out.println("数据库连接正常");
		} else {
			System.out.println("数据库连接失败");
		}
	}

}
