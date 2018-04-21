package net.zengzhiying.controller;

import java.io.IOException;
import java.io.PrintWriter;
import java.util.List;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.RequestMapping;

import net.zengzhiying.service.LoggerService;
import net.zengzhiying.service.StaffService;

/**
 * 用户设置控制器包括密码修改和日志查看
 * @author zengzhiying
 *
 */
@Controller("ConfigsController")
@RequestMapping("/webadmin/configs")
public class ConfigsController {
	
	/**
	 * 密码修改
	 * @return
	 * @throws IOException 
	 */
	@RequestMapping("/editpasswd")
	public String eidtPassword(HttpServletRequest request, HttpServletResponse response) throws IOException {
		if(request.getSession().getAttribute("user") != null) {
			if(request.getMethod().equals("POST")) {
				PrintWriter out = response.getWriter();
				StaffService ss = new StaffService();
				//修改密码处理
				String old_password = request.getParameter("old_password");
				String new_password = request.getParameter("new_password");
				String repeat_password = request.getParameter("repeat_password");
				
				String username = (String) request.getSession().getAttribute("user");
				//交给Service处理
				out.print(ss.editPasswd(old_password, new_password, repeat_password, username));
				return null;
			} else {
				//修改密码界面
				return "editpassword";
			}
			
		} else {
			return "error/superError";
		}
		
	}
	
	/**
	 * 系统日志查看
	 * @return
	 */
	@RequestMapping("/logs.view")
	public String showLogs(HttpServletRequest request) {
		if(request.getSession().getAttribute("super_type") != null) {
			//只有超级管理员能查看
			//查询日志库，默认只查看前100条
			LoggerService ls = new LoggerService();
			List<Object> logsList = ls.logsList();
			request.setAttribute("logsList", logsList);
			return "logs";
		} else {
			return "error/superError";
		}
	}
	
}
