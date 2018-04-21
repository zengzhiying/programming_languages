package net.zengzhiying.service;

import net.zengzhiying.beans.User;
import net.zengzhiying.dao.BeansDao;
import net.zengzhiying.tools.DataEncrypt;

/**
 * 用户登录服务类
 * @author zengzhiying
 *
 */

public class LoginService {
	
	/**
	 * 判断用户提交的登录信息是否正确
	 * @param u
	 * @return
	 */
	public static String userLogin(User u) {
		//加密密码
		u.setPassword(new DataEncrypt().shaEncrypt(u.getPassword()));
		BeansDao bd = new BeansDao();
		User outUser = (User) bd.queryOne("User", "userLogin", u);
		if(outUser == null) {
			//没有查询到
			return "error";
		} else if(outUser.getAccess_type() == 1) {
			// 超级管理员
			return "boss";
		} else if(outUser.getAccess_type() == 2){
			// 普通管理员
			return "admin";
		} else {
			//普通用户 不予登录
			return "error";
		}
	}
	
}
