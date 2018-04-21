package net.zengzhiying.service;

import java.util.List;

import org.hibernate.Query;
import org.hibernate.Session;
import org.hibernate.Transaction;

import net.zengzhiying.beans.Notice;
import net.zengzhiying.db.HibernateUtil;
import net.zengzhiying.tools.DataTypeConver;

/**
 * 公告管理服务类
 * @author zengzhiying
 *
 */

public class NoticeService {
	
	/**
	 * 查询所有公告
	 * @return
	 */
	@SuppressWarnings("unchecked")
	public List<Notice> noticeList() {
		
		//获得session并开启事务
		Session session = HibernateUtil.getSession();
		Transaction action = session.beginTransaction();
		String hql = "from Notice nt where 1=1";
		List<Notice> ntList = session.createQuery(hql).list();
		
		//提交事务并注销session
		action.commit();
		HibernateUtil.logoutSession(session);
		
		
		return ntList;
	}
	
	/**
	 * 按条查询公告
	 * @param nt_id
	 * @return
	 */
	@SuppressWarnings("unchecked")
	public Notice noticeOne(int nt_id) {
		
		//获得session并开启事务
		Session session = HibernateUtil.getSession();
		Transaction action = session.beginTransaction();
		String hql = "from Notice nt where nt.nt_id=?";
		List<Notice> ntList = session.createQuery(hql).setInteger(0, nt_id).list();
		Notice nt = ntList.get(0);
		
		
		//提交事务并注销session
		action.commit();
		HibernateUtil.logoutSession(session);
		
		return nt;
	}
	
	/**
	 * 更新公告内容
	 * @param nt_id
	 * @return
	 */
	public String updateNoticeOne(int nt_id,String nt_title,String nt_desc,String nt_content) {
		//获得session并开启事务
		Session session = HibernateUtil.getSession();
		Transaction action = session.beginTransaction();
		String hql = "update Notice nt set nt.nt_title=?, nt.nt_desc=?, nt.nt_content=? where nt.nt_id=?";
		Query update = session.createQuery(hql);
		update.setString(0, nt_title);
		update.setString(1, nt_desc);
		update.setString(2, nt_content);
		update.setInteger(3, nt_id);
		int n = update.executeUpdate();
		
		//提交事务并注销session
		action.commit();
		HibernateUtil.logoutSession(session);
		if(n == 1) {
			return "success";
		} else {
			return "error";
		}
	}
	
	
	public static void main(String[] args) {
		System.out.println(new NoticeService().updateNoticeOne(1, "1", "12", "123"));
	}
	
	/**
	 * 添加新公告内容
	 * @return
	 */
	public String addNotice(String nt_title,String nt_desc,String nt_content) {
		Notice nt = new Notice();
		nt.setNt_title(nt_title);
		nt.setNt_desc(nt_desc);
		nt.setNt_content(nt_content);
		nt.setNt_time(new DataTypeConver().newTime());
		//获得session并开启事务
		Session session = HibernateUtil.getSession();
		Transaction action = session.beginTransaction();
		
		session.save(nt);
		//提交事务并注销session
		action.commit();
		HibernateUtil.logoutSession(session);
				
		
		return "success";
	}
	
	public String delete(int nt_id) {
		//获得session并开启事务
		Session session = HibernateUtil.getSession();
		Transaction action = session.beginTransaction();
		String hql = "delete from Notice nt where nt.nt_id=?";
		int n = session.createQuery(hql).setInteger(0, nt_id).executeUpdate();
		//提交事务并注销session
		action.commit();
		HibernateUtil.logoutSession(session);
		if(n == 1) {
			return "success";
		} else {
			return "filed_error";
		}
	}
	
}
