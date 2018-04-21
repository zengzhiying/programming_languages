package net.zengzhiying.service;

import java.util.List;

import net.zengzhiying.dao.BeansDao;
import net.zengzhiying.tools.DataTypeConver;

/**
 * 首页相关数据查询类
 * @author zengzhiying
 *
 */

public class IndexService {
	
	/**
	 * 本周新增员工数量计算
	 * @return
	 */
	public int newStaffNumber() {
		DataTypeConver dtc = new DataTypeConver();
		BeansDao bd = new BeansDao();
		//获得7日之前时间戳
		long on_time = dtc.newTime() - 7 * 24 * 3600;
		// 查询之后的员工数
		List<Object> staffList = bd.queryList("User", "latelyUserNumber", on_time);
		return staffList.size();
	}
	
	/**
	 * 查询公司中除了boss之外所有的员工数
	 * @return
	 */
	public int countStaffNumber() {
		List<Object> staffList = new BeansDao().queryList("User", "countUserNumber");
		return staffList.size();
	}
	
	/**
	 * 获取部门个数
	 * @return
	 */
	public int countPartmentNumber() {
		List<Object> partmentList = new BeansDao().queryList("Partment", "queryAll");
		
		return partmentList.size();
	}
	
	public static void main(String[] args) {
		new IndexService().newStaffNumber();
	}
	
}
