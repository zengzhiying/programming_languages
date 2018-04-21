package net.zengzhiying.db;

import org.hibernate.Session;
import org.hibernate.SessionFactory;
import org.hibernate.Transaction;
import org.hibernate.boot.registry.StandardServiceRegistry;
import org.hibernate.boot.registry.StandardServiceRegistryBuilder;
import org.hibernate.cfg.Configuration;

/**
 * Hibernate工具类
 * @author zengzhiying
 *
 */

public class HibernateUtil {
	
	private static SessionFactory sessionFactory;
	private static Configuration config;
	private static StandardServiceRegistryBuilder ssrb;
	private static StandardServiceRegistry ssr;
    static {
    	//创建配置对象
        config = new Configuration().configure();
        //创建服务注册对象
        ssrb = new StandardServiceRegistryBuilder().applySettings(config.getProperties());
        ssr = ssrb.build();
        //创建会话工厂对象
        sessionFactory = config.buildSessionFactory(ssr);
    }
    
    /**
     * 获取sessionfactory会话工厂
     * @return
     */
    public static SessionFactory getSessionFactory() {
        return sessionFactory;
    }
    
    /**
     * 获取会话对象
     * @return
     */
    public static Session getSession() {
    	//sessionFactory = config.buildSessionFactory(ssr);
    	Session session = sessionFactory.openSession();
        return session;
    }
    
    /**
     * 关闭session 操作完之后要及时关闭session否则会资源占用过满，session关闭不会影响性能
     * 每一个业务逻辑操作中，获取需要的数据之后如果可能，尽量关闭session后再进行耗时比较长的计算，用来提高并发
     * @param session
     */
    public static void logoutSession(Session session) {
        if(session != null) {
        	session.flush();
    		session.close();
        }
    }
	
}
