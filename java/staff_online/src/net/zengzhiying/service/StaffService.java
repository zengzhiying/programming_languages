package net.zengzhiying.service;

import java.util.List;

import net.zengzhiying.beans.Partment;
import net.zengzhiying.beans.User;
import net.zengzhiying.dao.BeansDao;
import net.zengzhiying.tools.DataEncrypt;
import net.zengzhiying.tools.DataTypeConver;
import net.zengzhiying.tools.DataValidation;

/**
 * 员工操作服务类 包括部门经理和普通员工
 * @author zengzhiying
 *
 */

public class StaffService {
	
	/**
	 * 员工列表查询
	 * @return
	 */
	public List<Object> staffList(int access_type) {
		
		BeansDao bd = new BeansDao();
		List<Object> userList = bd.queryList("User", "staffList", access_type);
		return userList;
		
	}
	
	
	/**
	 * 根据username查询部门id
	 * @param username
	 * @return
	 */
	public int getPartmentId(String username) {
		BeansDao bd = new BeansDao();
		User user = (User) bd.queryOne("User", "queryUser1", username);
		if(user != null) {
			// 正常情况肯定不为空
			return user.getDepartment_id();
		}
		return 0;
	}
	
	/**
	 * 根据部门id查询部门名称
	 * @param partment_id
	 * @return
	 */
	public Partment getPartment(int partment_id) {
		BeansDao bd = new BeansDao();
		Partment pm = (Partment) bd.queryOne("Partment", "queryName", partment_id);
		return pm;
	}
	
	/**
	 * 新增员工
	 * @param user_name
	 * @param gender
	 * @param age
	 * @param partment_id
	 * @param contact
	 * @param user_describe
	 * @return
	 */
	public String addPartment(String user_name, String gender,String age,String partment_id,String contact,String user_describe,int is_approve) {
		DataValidation dv = new DataValidation();
		BeansDao bd = new BeansDao();
		DataTypeConver dtc = new DataTypeConver();
		User u = new User();
		if(user_name == null || user_name.equals("") || 
				gender == null || gender.equals("") || 
				age == null || age.equals("") ||
				partment_id == null || partment_id.equals("") ||
				contact == null || contact.equals("")) {
			return "empty";
		}
		if(user_describe == null || user_describe.equals("")) {
			user_describe = "0";
		}
		if(!gender.equals("0") && !gender.equals("1")) {
			return "param_error";
		}
		if(!dv.posInteger(age) || !dv.posInteger(partment_id)) {
			return "param_error";
		}
		if(Integer.valueOf(age) > 100) {
			return "age_error";
		}
		
		//拼装对象
		u.setUser_name(user_name);
		u.setGender(Integer.valueOf(gender));
		u.setAge(Integer.valueOf(age));
		u.setDepartment_id(Integer.valueOf(partment_id));
		u.setContact(contact);
		u.setIs_approve(is_approve);
		u.setAccess_type(3);
		u.setUsername("0");
		u.setPassword("0");
		u.setCommit_time(dtc.newTime());
		if(is_approve == 0) {
			u.setRegister_time(0);
		} else {
			u.setRegister_time(dtc.newTime());
		}
		u.setUser_describe(user_describe);
		
		//插入操作
		if(bd.insertOne("User", "insertUser", u)) {
			return "success";
		}
		return "error";
	}
	
	
	/**
	 * 新增员工 - 重载
	 * @param user_name
	 * @param gender
	 * @param age
	 * @param partment_id
	 * @param contact
	 * @param user_describe
	 * @return
	 */
	public String addPartment(String user_name, String gender,String age,String partment_id,String contact,String user_describe,int is_approve,String user_id) {
		DataValidation dv = new DataValidation();
		BeansDao bd = new BeansDao();
		User u = new User();
		if(user_name == null || user_name.equals("") || 
				gender == null || gender.equals("") || 
				age == null || age.equals("") ||
				partment_id == null || partment_id.equals("") ||
				contact == null || contact.equals("") ||
				user_id == null || user_id.equals("")) {
			return "empty";
		}
		if(user_describe == null || user_describe.equals("")) {
			user_describe = "0";
		}
		if(!gender.equals("0") && !gender.equals("1")) {
			return "param_error";
		}
		if(!dv.posInteger(user_id) || !dv.posInteger(age) || !dv.posInteger(partment_id)) {
			return "param_error";
		}
		if(Integer.valueOf(age) > 100) {
			return "age_error";
		}
		//拼装对象
		u.setUser_id(Integer.valueOf(user_id));
		u.setUser_name(user_name);
		u.setGender(Integer.valueOf(gender));
		u.setAge(Integer.valueOf(age));
		u.setDepartment_id(Integer.valueOf(partment_id));
		u.setContact(contact);
		u.setIs_approve(is_approve);
		//u.setAccess_type(3);
		//u.setUsername("0");
		//u.setPassword("0");
		//u.setCommit_time(dtc.newTime());
//		if(is_approve == 0) {
//			u.setRegister_time(0);
//		} else {
//			u.setRegister_time(dtc.newTime());
//		}
		u.setUser_describe(user_describe);
		//更新操作
		if(bd.updateOne("User", "updateUser", u)) {
			return "success";
		}
		return "error";
	}
	
	
	/**
	 * 新增部门经理 
	 * @param user_name
	 * @param gender
	 * @param age
	 * @param partment_id
	 * @param contact
	 * @param user_describe
	 * @return
	 */
	public String addManagePartment(String user_name, String gender,String age,String partment_id,String contact,String user_describe,int is_approve,String username,String password) {
		DataValidation dv = new DataValidation();
		DataTypeConver dtc = new DataTypeConver();
		BeansDao bd = new BeansDao();
		User u = new User();
		if(user_name == null || user_name.equals("") || 
				gender == null || gender.equals("") || 
				age == null || age.equals("") ||
				partment_id == null || partment_id.equals("") ||
				contact == null || contact.equals("") ||
				username == null || username.equals("") ||
				password == null || password.equals("")) {
			return "empty";
		}
		if(user_describe == null || user_describe.equals("")) {
			user_describe = "0";
		}
		if(!gender.equals("0") && !gender.equals("1")) {
			return "param_error";
		}
		if(!dv.posInteger(age) || !dv.posInteger(partment_id)) {
			return "param_error";
		}
		if(Integer.valueOf(age) > 100) {
			return "age_error";
		}
		//username password合法性校验
		if(!dv.usernameValidation(username)) {
			return "username_error";
		}
		if(!dv.passwordValidation(password)) {
			return "password_error";
		}
		
		// 用户名防止重复逻辑
		if(bd.queryList("User", "queryUser1",username).size() > 0) {
			return "username_repeat";
		}
		
		//加密密码
		password = new DataEncrypt().shaEncrypt(password);
		//拼装对象
		u.setUser_name(user_name);
		u.setGender(Integer.valueOf(gender));
		u.setAge(Integer.valueOf(age));
		u.setDepartment_id(Integer.valueOf(partment_id));
		u.setContact(contact);
		u.setIs_approve(is_approve);
		u.setAccess_type(2);
		u.setUsername(username);
		u.setPassword(password);
		u.setCommit_time(dtc.newTime());
		if(is_approve == 0) {
			u.setRegister_time(0);
		} else {
			u.setRegister_time(dtc.newTime());
		}
		u.setUser_describe(user_describe);
		//更新操作
		if(bd.insertOne("User", "insertUser", u)) {
			return "success";
		}
		return "error";
	}
	
	
	/**
	 * 更新部门经理  - 重载
	 * @param user_name
	 * @param gender
	 * @param age
	 * @param partment_id
	 * @param contact
	 * @param user_describe
	 * @return
	 */
	public String addManagePartment(String user_name, String gender,String age,String partment_id,String contact,String user_describe,int is_approve,String username,String password,String user_id) {
		DataValidation dv = new DataValidation();
		DataTypeConver dtc = new DataTypeConver();
		BeansDao bd = new BeansDao();
		User u = new User();
		if(user_name == null || user_name.equals("") || 
				gender == null || gender.equals("") || 
				age == null || age.equals("") ||
				partment_id == null || partment_id.equals("") ||
				contact == null || contact.equals("") ||
				username == null || username.equals("") ||
				user_id == null || user_id.equals("")) {
			return "empty";
		}
		if(user_describe == null || user_describe.equals("")) {
			user_describe = "0";
		}
		if(!gender.equals("0") && !gender.equals("1")) {
			return "param_error";
		}
		if(!dv.posInteger(user_id) || !dv.posInteger(age) || !dv.posInteger(partment_id)) {
			return "param_error";
		}
		if(Integer.valueOf(age) > 100) {
			return "age_error";
		}
		//username password合法性校验
		if(!dv.usernameValidation(username)) {
			return "username_error";
		}
		
		
		
		// 用户名防止重复逻辑 除了自己之外
		List<Object> objList = bd.queryList("User", "queryUser1",username);
		for(Object obj:objList) {
			User u1 = (User) obj;
			if(u1.getUsername().equals(username) && u1.getUser_id() == Integer.valueOf(user_id)) {
				// 正常放行
			} else {
				return "username_repeat";
			}
		}
		
		int i = 0;
		//密码处理逻辑
		if(password != null && !password.equals("")) {
			if(!dv.passwordValidation(password)) {
				return "password_error";
			} else {
				// 加密密码
				password = new DataEncrypt().shaEncrypt(password);
				u.setPassword(password);
				i = 1;
			}
		}
		
		//拼装对象
		u.setUser_id(Integer.valueOf(user_id));
		u.setUser_name(user_name);
		u.setGender(Integer.valueOf(gender));
		u.setAge(Integer.valueOf(age));
		u.setDepartment_id(Integer.valueOf(partment_id));
		u.setContact(contact);
		u.setIs_approve(is_approve);
		u.setAccess_type(2);
		//u.setCommit_time(dtc.newTime());
		u.setUsername(username);
		u.setUser_describe(user_describe);
		if(i == 0) {
			if(bd.updateOne("User", "updateUser2", u)) {
				return "success";
			}
			return "error";
		} else {
			//更新操作
			if(bd.updateOne("User", "updateUser1", u)) {
				return "success";
			}
			return "error";
		}
		
	}
	
	/**
	 * 获取单条员工信息
	 * @param user_id
	 * @return
	 */
	public User getUser(String user_id) {
		BeansDao bd = new BeansDao();
		DataValidation dv = new DataValidation();
		if(user_id == null || user_id.equals("")) {
			return null;
		}
		if(!dv.posInteger(user_id)) {
			return null;
		}
		User u = (User) bd.queryOne("User", "queryUser", Integer.valueOf(user_id));
		return u;
	}
	
	/**
	 * 删除普通员工和部门经理方法
	 * 部门经理权限在控制器锁定
	 * @param user_id
	 * @return
	 */
	public String deleteUser(String user_id) {
		BeansDao bd = new BeansDao();
		DataValidation dv = new DataValidation();
		if(!dv.posInteger(user_id)) {
			return "param_error";
		}
		int id = Integer.valueOf(user_id);
		User u = (User) bd.queryOne("User", "queryUser", id);
		if(u == null) {
			//没有该员工
			return "param_error";
		}
		if(u.getAccess_type() == 1) {
			// 超级管理员不能被删除
			return "super_error";
		}
		if(bd.deleteOne("User", "deleteUser", id)) {
			return "success";
		} else {
			return "error";
		}
	}
	
	/**
	 * 员工审批
	 * @param user_id
	 * @return
	 */
	public String userApprove(int user_id) {
		BeansDao bd = new BeansDao();
		if(bd.updateOne("User", "approveUser", user_id)) {
			return "success";
		} else {
			return "error";
		}
	}
	
	/**
	 * 员工密码修改
	 * @param old_password
	 * @param new_password
	 * @param repeat_password
	 * @return
	 */
	public String editPasswd(String old_password,String new_password,String repeat_password,String username) {
		if(old_password == null || new_password == null || repeat_password == null
				|| old_password.equals("") || new_password.equals("") || repeat_password.equals("")) {
			return "empty";
		}
		if(!new_password.equals(repeat_password)) {
			return "no_agr";
		}
		
		BeansDao bd = new BeansDao();
		User u = (User) bd.queryOne("User", "usernameToUser", username);
		DataEncrypt de = new DataEncrypt();
		DataValidation dv = new DataValidation();
		if(u != null) {
			//检验新密码是否合格
			if(!dv.passwordValidation(new_password)) {
				//密码不符合要求
				return "password_dv";
			}
			
			//加密比对原密码
			if(!de.shaEncrypt(old_password).equals(u.getPassword())) {
				//原密码不正确
				return "passwd_error";
			}
			
			// 密码完好 加密写入数据库
			User u1 = new User();
			u1.setUsername(username);
			u1.setPassword(de.shaEncrypt(new_password));
			if(bd.updateOne("User", "eidtUserToPass", u1)) {
				return "success";
			} else {
				return "error";
			}
			
		} else {
			//此用户不存在，一般是不可能的，建议将用户退出
		}
		
		return null;
	}
	
}
