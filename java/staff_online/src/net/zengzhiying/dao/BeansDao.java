package net.zengzhiying.dao;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

import org.apache.ibatis.session.SqlSession;

import net.zengzhiying.beans.User;
import net.zengzhiying.db.DBAccess;

/**
 * 对象数据持久层操作类
 * @author zengzhiying
 *
 */

public class BeansDao {
	//定义sqlSession
	private static SqlSession sqlSession = null;
	static {
		//单例模式实例化获取一个数据库连接
		try {
			if(sqlSession == null) {
				sqlSession = DBAccess.getSqlSession();
			}
		} catch (IOException e) {
			e.printStackTrace();
		}
	}
	
	/**
	 * 查询固定sql语句的数据
	 * @param sqlName 命名空间
	 * @param sqlId 指定编号的sql
	 * @return obj
	 */
    public List<Object> queryList(String sqlName, String sqlId) {
        List<Object> objList = new ArrayList<Object>();
        objList = sqlSession.selectList(sqlName+ "." + sqlId); 
        return objList;
    }
    
    
    /**
     * 查询符合条件的单条数据
     * @param objName
     * @param sqlId
     * @param sqlCmd
     * @return
     */
    public Object queryOne(String objName, String sqlId, String sqlCmd) {
    	Object outObj = sqlSession.selectOne(objName + "." + sqlId, sqlCmd);
    	return outObj;
    }
    
    /**
     * 查询符合条件的单条数据 - 重载1
     * @param objName
     * @param sqlId
     * @param sqlCmd
     * @return
     */
    public Object queryOne(String objName, String sqlId, int sqlCmd) {
    	Object outObj = sqlSession.selectOne(objName + "." + sqlId, sqlCmd);
    	return outObj;
    }
    
    /**
     * 查询符合条件的单条数据 - 重载2
     * @param objName
     * @param sqlId
     * @param inObj
     * @return
     */
    public Object queryOne(String objName, String sqlId, Object inObj) {
    	Object outObj = sqlSession.selectOne(objName + "." + sqlId, inObj);
    	return outObj;
    }
    
    /**
     * 查询符合条件的数据集
     * @param objName,sqlId,sqlCmd sql的参数
     * @return List<Message>
     */
    public List<Object> queryList(String objName ,String sqlId, String sqlCmd) {
        List<Object> messageList = new ArrayList<Object>();
        
        //执行sql
        messageList = sqlSession.selectList(objName + "." + sqlId, sqlCmd);
        return messageList;
    }
    
    /**
     * 传入整型数据查询符合条件的数据
     * @param objName
     * @param sqlId
     * @param sqlCmd
     * @return
     */
    public List<Object> queryList(String objName, String sqlId, int sqlCmd) {
    	List<Object> outObjList = new ArrayList<Object>();
    	//执行sql
    	outObjList = sqlSession.selectList(objName + "." + sqlId, sqlCmd);
    	return outObjList;
    }
    
    /**
     * 查询符合条件的数据集 - 重载
     * @param objName
     * @param sqlId
     * @param inObj
     * @return List<Object> obj
     */
    public List<Object> queryList(String objName, String sqlId, Object inObj) {
    	List<Object> outObjList = new ArrayList<Object>();
    	outObjList = sqlSession.selectList(objName + "." + sqlId, inObj);
    	return outObjList;
    }
    
    /**
     * 插入单条数据
     * @param objName
     * @param sqlId
     * @param inObj
     * @return
     */
    public boolean insertOne(String objName, String sqlId, Object inObj) {
    	if(sqlSession.insert(objName + "." + sqlId, inObj) == 1) {
    		sqlSession.commit();
        	return true;
    	} else {
    		return false;
    	}
    	
    }
    
    /**
     * 删除单条数据
     * @param objName
     * @param sqlId
     * @param id
     * @return
     */
    public boolean deleteOne(String objName, String sqlId,int id) {
    	if(sqlSession.delete(objName + "." + sqlId, id) == 1) {
    		sqlSession.commit();
    		return true;
    	} else {
    		return false;
    	}
    }
    
    /**
     * 更新单条数据
     * @param objName
     * @param sqlId
     * @param obj
     * @return
     */
    public boolean updateOne(String objName, String sqlId, Object obj) {
    	if(sqlSession.update(objName + "." + sqlId, obj) == 1) {
    		sqlSession.commit();
    		return true;
    	}
    	return false;
    }
    
    /**
     * 更新单条数据 - 重载
     * @param objName
     * @param sqlId
     * @param obj
     * @return
     */
    public boolean updateOne(String objName, String sqlId, int user_id) {
    	if(sqlSession.update(objName + "." + sqlId, user_id) == 1) {
    		sqlSession.commit();
    		return true;
    	}
    	return false;
    }
    
    public static void main(String[] args) {
//		User u = new User();
//		u.setUsername("admin1");
//		u.setPassword("d033e22ae348aeb5660fc2140aec35850c4da997");
//		BeansDao bd = new BeansDao();
//		//查询没有不会抛异常
//		User u1 = (User) bd.queryOne("User","userLogin" ,u);
//		if(u1 == null) {
//			System.out.println("没查询到");
//		}
    	BeansDao bd = new BeansDao();
    	//bd.queryOne("User", "u", 7);
//    	List<Object> objList = bd.queryList("User", "queryUser1","admin12");
//    	for(Object obj:objList) {
//    		User u = (User) obj;
//    		System.out.println(u.toString());
//    	}
		//System.out.println(u1.toString());
	}
	
	
}
