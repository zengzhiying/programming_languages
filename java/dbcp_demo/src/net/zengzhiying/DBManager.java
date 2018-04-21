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
 * 通过构造方法传入dbcp配置文件 用于灵活的使用多个连接
 * @author monchickey
 *
 */

public class DBManager {
    private static final Log LOG = LogFactory.getLog(DBManager.class);
    
    private DataSource dataSource;
    
    public DBManager(String configFile) {
        Properties dbcpProperties = new Properties();
        try {
            dbcpProperties.load(DBManager.class.getClassLoader().getResourceAsStream(configFile));
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
    
    /**
     * 获取数据库连接
     * @return
     */
    public Connection getConnection() {
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
    public void closeConnection(Connection conn) {
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
