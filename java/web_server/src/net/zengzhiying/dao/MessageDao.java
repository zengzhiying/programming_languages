package net.zengzhiying.dao;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

import org.apache.ibatis.session.SqlSession;

import net.zengzhiying.beans.Message;
import net.zengzhiying.db.DBAccess;

/**
 * 数据库访问层实例 用于编写通用数据库层与service层交互的逻辑
 * @author Administrator
 *
 */

public class MessageDao {
	
	//定义sqlSession
	private static SqlSession sqlSession = null;
	
	static {
		//单例模式实例化
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
	 * @param sqlId
	 * @return
	 */
	public List<Message> queryList(String sqlId) {
		List<Message> messageList = new ArrayList<Message>();
		
		messageList = sqlSession.selectList(sqlId);
		
		return messageList;
	}
	
	/**
	 * 查询符合条件的数据集
	 * @param sqlId,sqlCmd sql的参数
	 * @return List<Message>
	 */
	public List<Message> queryList(String sqlId, String sqlCmd) {
		List<Message> messageList = new ArrayList<Message>();
		
		//执行sql
		messageList = sqlSession.selectList("Message." + sqlId, sqlCmd);
		return messageList;
	}
	
	
	/**
	 * 查询结果集的一条数据，而且条件是只能查一条
	 * @param sqlId
	 * @return Message msg
	 */
	public Message queryOne(String sqlId) {
		Message msg = new Message();
		//查一条
		msg = sqlSession.selectOne("Message." + sqlId);
		return msg;
	}
	
	/**
	 * 按照对象查询
	 * @param sqlId
	 * @param msg
	 * @return
	 */
	public List<Message> queryList(String sqlId, Message msg) {
		List<Message> messageList = new ArrayList<Message>();
		//按照对象查询
		messageList = sqlSession.selectList(sqlId, msg);
		return messageList;
	}
	
	/**
	 * 删除单条信息
	 * @param sqlId
	 * @param id
	 */
	public void deleteOne(String sqlId, int id) {
		sqlSession.delete(sqlId, id);
		//删除事务提交
		sqlSession.commit();
	}
	
	/**
	 * 更新单条数据，传入新对象
	 * @param sqlId
	 * @param newMsg
	 */
	public void updateOne(String sqlId, Message newMsg) {
		sqlSession.update(sqlId, newMsg);
		sqlSession.commit();
	}
	
	/**
	 * 批量删除数据，传入List对象
	 * @param sqlId
	 * @param messageList
	 */
	public void deleteBatch(String sqlId, List<Integer> intList) {
		sqlSession.delete(sqlId, intList);
		sqlSession.commit();
	}
	
	/**
	 * 添加一条数据
	 * @param sqlId
	 * @param msg
	 */
	public void insertOne(String sqlId, Message msg) {
		sqlSession.insert(sqlId, msg);
		sqlSession.commit();
	}
	
	/**
	 * 批量添加数据，最后再提交事务
	 * @param sqlId
	 * @param msgList
	 */
	public void insertBatch(String sqlId, List<Message> msgList) {
		for(Message msg:msgList) {
			sqlSession.insert(sqlId, msg);
		}
		//提交事务
		sqlSession.commit();
	}
	
}
