package services;

import java.sql.Connection;
import java.sql.PreparedStatement;
import java.sql.SQLException;

import entity.Website;
import utils.DBConn;

/*
 * 数据库操作业务逻辑
 */

public class DbOperation {
	
	private static Connection conn = null;
	
	public DbOperation() {
		conn = DBConn.getConnection();
	}
	
	public boolean addData(Website w) {
		String title = w.getTitle();
		String link = w.getLink();
		String classify = w.getClassify();
		int sort = w.getSort();
		
		String sql = "INSERT INTO `website_web` (`id`, `title`, `link`, `classify`, `sort`) VALUES (NULL, ?, ?, ?, ?);";
		
		
		try {
			//预编译
			PreparedStatement ptmt = conn.prepareStatement(sql);
			//传参
			ptmt.setString(1, title);
			ptmt.setString(2, link);
			ptmt.setString(3, classify);
			ptmt.setInt(4, sort);
			//执行
			if(!(ptmt.execute())) {
				return true;
			} else {
				return false;
			}
		} catch (SQLException e) {
			
			e.printStackTrace();
		}
		
		return false;
	}

	public static void main(String[] args) {
		//测试
//		Website ws = new Website("无忧演讲","http://www.51yjg.com/", "综合资源", 6);
//		if(new DbOperation().addData(ws)) {
//			System.out.println("新增成功");
//		} else {
//			System.out.println("新增失败");
//		}
	}

}
