package net.zengzhiying.service;

import java.util.List;

import org.hibernate.Query;
import org.hibernate.Session;
import org.hibernate.Transaction;

import net.zengzhiying.beans.CheckWork;
import net.zengzhiying.db.HibernateUtil;
import net.zengzhiying.tools.DataTypeConver;

/**
 * 员工考勤服务类
 * @author zengzhiying
 *
 */

public class CheckWorkService {
	
	/**
	 * 获得本月所有签到数据
	 * @return
	 */
	@SuppressWarnings("unchecked")
	public List<CheckWork> workData(int user_id, int i) {
		if(i == 1) {
			//获得session 并开启事务
			Session session = HibernateUtil.getSession();
			Transaction action = session.beginTransaction();
			//实例化相关类
			DataTypeConver dtc = new DataTypeConver();
			//获得当前月份
			String mon = dtc.timeToString(dtc.newTime(), "yyyyMM");
			//构建查询
			String hql = "from CheckWork checkwork where checkwork.user_id=? and checkwork.sign_date like ?";
			Query query = session.createQuery(hql);
			query.setInteger(0, user_id);
			query.setString(1, "%" + mon + "%"); //查询直接用字符串代替即可，不用外加单引号
			List<CheckWork> ckList = query.list();
			//提交事务
			action.commit();
			//注销session
			HibernateUtil.logoutSession(session);
			
			//返回结果
			return ckList;
		} else {
			//获得session 并开启事务
			Session session = HibernateUtil.getSession();
			Transaction action = session.beginTransaction();
			//实例化相关类
			DataTypeConver dtc = new DataTypeConver();
			//获得当前年月
			//String mon = dtc.timeToString(dtc.newTime(), "yyyyMM");
			//获得当前日期
			int day = Integer.valueOf(dtc.timeToString(dtc.newTime(), "dd"));
			//前推时间戳获得上个月
			String old_mon = dtc.timeToString(dtc.newTime() - day*3600*24, "yyyyMM");
			//构建查询
			String hql = "from CheckWork checkwork where checkwork.user_id=? and checkwork.sign_date like ?";
			Query query = session.createQuery(hql);
			query.setInteger(0, user_id);
			query.setString(1, "%" + old_mon + "%"); //查询直接用字符串代替即可，不用外加单引号
			List<CheckWork> ckList = query.list();
			//提交事务
			action.commit();
			//注销session
			HibernateUtil.logoutSession(session);
			
			//返回结果
			return ckList;
		}
		
	}
	
	public static void main(String[] args) {
		System.out.println(new CheckWorkService().workData(2,2));
	}
	
}
