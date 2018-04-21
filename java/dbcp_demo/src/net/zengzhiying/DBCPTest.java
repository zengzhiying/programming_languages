package net.zengzhiying;

import java.sql.Connection;
import java.sql.PreparedStatement;
import java.sql.ResultSet;
import java.sql.SQLException;

/**
 * 连接池的普通jdbc连接测试对比
 * @author Administrator
 *
 */

public class DBCPTest {
    public static void main(String[] args) {
        Connection conn1 = null;
        conn1 = FixedDBManager.getConnection();
        for(int i = 1;i <= 150000;i++) {
            
            PreparedStatement pstmt;
            ResultSet rs;
            try {
                pstmt = conn1.prepareStatement("SELECT * FROM number_type WHERE type_id>=?");
                pstmt.setInt(1, 1);
                rs = pstmt.executeQuery();
                while(rs.next()) {
                    System.out.println(rs.getString("type_name") + ":"
                            + rs.getString("type_describe"));
                }
            } catch (SQLException e) {
                System.out.println("sql执行异常！");
                e.printStackTrace();
            }
            System.out.println(i);
            
        }
        FixedDBManager.closeConnection(conn1);
        
//        conn1 = MysqlConnectionService.getConnection("localhost", 3306, "phone_number", "root", "root");
//        for(int i = 1;i <= 150000;i++) {
//            
//            PreparedStatement pstmt;
//            ResultSet rs;
//            try {
//                pstmt = conn1.prepareStatement("SELECT * FROM number_type WHERE type_id>=?");
//                pstmt.setInt(1, 1);
//                rs = pstmt.executeQuery();
//                while(rs.next()) {
//                    System.out.println(rs.getString("type_name") + ":"
//                            + rs.getString("type_describe"));
//                }
//            } catch (SQLException e) {
//                System.out.println("sql执行异常！");
//                e.printStackTrace();
//            }
//            System.out.println(i);
//            
//        }
//        MysqlConnectionService.close(conn1);
    }
}
