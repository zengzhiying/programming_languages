package net.zengzhiying.tests;

import java.io.IOException;

import org.apache.ibatis.session.SqlSession;

import net.zengzhiying.db.DBAccess;

public class MysqlConnectTest {
	public static void main(String[] args) {
		try {
			SqlSession ss = DBAccess.getSqlSession();
			if(ss != null) {
				System.out.println("databse connect ok...");
			} else {
				System.out.println("database connect error....");
			}
		} catch (IOException e) {
			e.printStackTrace();
		}
	}
}
