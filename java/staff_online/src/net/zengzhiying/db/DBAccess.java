package net.zengzhiying.db;

import java.io.IOException;
import java.io.Reader;

import org.apache.ibatis.io.Resources;
import org.apache.ibatis.session.SqlSession;
import org.apache.ibatis.session.SqlSessionFactory;
import org.apache.ibatis.session.SqlSessionFactoryBuilder;

/**
 * 数据库连接层 提供mysql数据库底层连接
 * @author zengzhiying
 *
 */

public class DBAccess {
	
	public static SqlSession getSqlSession() throws IOException {
        //通过配置文件获取数据库连接
        Reader reader = Resources.getResourceAsReader("net/zengzhiying/configs/Configuration.xml");
        //构建SqlSessionFactory
        SqlSessionFactory sqlSessionFactory = new SqlSessionFactoryBuilder().build(reader);
        //打开数据库会话
        SqlSession sqlSession = sqlSessionFactory.openSession();
        return sqlSession;
    }
	
}
