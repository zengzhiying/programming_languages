package net.zengzhiying;

import java.io.IOException;
import java.sql.Connection;
import java.sql.DatabaseMetaData;
import java.sql.SQLException;
import java.util.Properties;

import javax.sql.DataSource;

import org.apache.commons.dbcp2.BasicDataSourceFactory;
import org.apache.commons.logging.Log;
import org.apache.commons.logging.LogFactory;

/**
 * 固定配置的dbmanager写法 只能使用配置文件配置的一个连接 不能切换
 * @author monchickey
 *
 */

public class FixedDBManager {
    private static final Log LOG = LogFactory.getLog(FixedDBManager.class);
    private static final String CONFIG_FILE = "net/zengzhiying/dbcp.properties";
    
    private static DataSource dataSource;
    
    static {
        Properties dbcpProperties = new Properties();
        try {
            dbcpProperties.load(FixedDBManager.class.getClassLoader().getResourceAsStream(CONFIG_FILE));
            dataSource = BasicDataSourceFactory.createDataSource(dbcpProperties);
            Connection conn = getConnection();
            if(conn != null) {
                DatabaseMetaData dbMetaData = conn.getMetaData();
                LOG.info("Connection to: " + dbMetaData.getDatabaseProductName() + " "
                + dbMetaData.getDatabaseProductVersion());
                closeConnection(conn);
            } else {
                LOG.error("获取到的连接为空！");
            }
        } catch (IOException e) {
            // e.printStackTrace();
            LOG.error("加载配置文件失败！");
        } catch (Exception e) {
            LOG.error("初始化连接池失败！");
            e.printStackTrace();
        }
    }
    
    private FixedDBManager() {
        
    }
    
    /**
     * 获取数据库连接
     * @return
     */
    public static Connection getConnection() {
        Connection conn = null;
        try {
            conn = dataSource.getConnection();
        } catch (SQLException e) {
            LOG.error("获取数据库连接失败！");
            e.printStackTrace();
        }
        return conn;
    }
    
    /**
     * 关闭数据库连接
     * @param conn
     */
    public static void closeConnection(Connection conn) {
        try {
            if(conn != null && !conn.isClosed()) {
                conn.setAutoCommit(true);
                conn.close();
            }
        } catch (SQLException e) {
            LOG.error("关闭数据库连接失败！");
            e.printStackTrace();
        }
    }
    
}
