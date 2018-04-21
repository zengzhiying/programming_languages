package net.zengzhiying.controller;

import java.io.IOException;
import java.io.PrintWriter;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;
import org.springframework.web.bind.annotation.ResponseBody;

import net.zengzhiying.beans.User;
import net.zengzhiying.service.IndexService;
import net.zengzhiying.service.LoginService;
import net.zengzhiying.tools.FilesHandle;

/**
 * 首页默认页管理
 * @author zengzhiying
 *
 */

@Controller("IndexController")
@RequestMapping(value={"/webadmin"})
public class IndexController {

	//首页输出
	@RequestMapping(value={"/",""},method=RequestMethod.GET)
	public String index(HttpServletRequest request, HttpServletResponse response) {
		if(request.getRequestURI().equals(request.getContextPath() + "/webadmin")) {
			try {
				response.sendRedirect(request.getContextPath() + "/webadmin/");
			} catch (IOException e) {
				e.printStackTrace();
			}
			return null;
		}
		//查询相关数据
		IndexService is = new IndexService();
		request.setAttribute("newNumber", is.newStaffNumber());
		request.setAttribute("countNumber", is.countStaffNumber());
		request.setAttribute("access_number", new FilesHandle().fileRead("/developers/Javaweb/StaffOnLine/data/login_access.data"));
		request.setAttribute("partment_number", is.countPartmentNumber());
		return "index";
	}
	
	@RequestMapping(value={"/login"})
	public String login(HttpServletRequest request, HttpServletResponse response,User user) {
		//System.out.println(user.toString());
		if(request.getMethod().equals("POST")) {
			if(user.getUsername() == null || user.getPassword() == null || user.getUsername().equals("") || user.getPassword().equals("")) {
				return "login_error";
			} else {
				//登录对比
				String loginStatus = LoginService.userLogin(user);
				if(loginStatus.equals("error")) {
					return "login_error";
				} else if(loginStatus.equals("boss")) {
					request.getSession().setAttribute("user", user.getUsername());
					request.getSession().setAttribute("super_type", user.getAccess_type());
					
					//登录成功 后台写文件统计次数
					FilesHandle fh = new FilesHandle();
					String loginNumber = fh.fileRead("/developers/Javaweb/StaffOnLine/data/login_access.data");
					int lN = Integer.valueOf(loginNumber) + 1;
					fh.fileWrite("/developers/Javaweb/StaffOnLine/data/login_access.data", String.valueOf(lN));
					// 写文件完毕
					return "login_access";
				} else if(loginStatus.equals("admin")) {
					request.getSession().setAttribute("user", user.getUsername());
					
					//登录成功 后台写文件统计次数
					FilesHandle fh = new FilesHandle();
					String loginNumber = fh.fileRead("/developers/Javaweb/StaffOnLine/data/login_access.data");
					int lN = Integer.valueOf(loginNumber) + 1;
					fh.fileWrite("/developers/Javaweb/StaffOnLine/data/login_access.data", String.valueOf(lN));
					// 写文件完毕
					return "login_access";
				} else {
					return "login_error";
				}
			}
		} else {
			//先注销session
			request.getSession().invalidate();
			return "login";
		}
	}
	
	/**
	 * 退出系统
	 * @return
	 * @throws IOException 
	 * @throws InterruptedException 
	 */
	@RequestMapping("/logout")
	public String logout(HttpServletRequest request, HttpServletResponse response) throws IOException, InterruptedException {
		if(request.getSession().getAttribute("user") != null || request.getSession().getAttribute("super_type") != null) {
			//销毁session
			request.getSession().invalidate();
		}
		PrintWriter pw = response.getWriter();
		pw.println("正在退出....");
		//response.sendRedirect(request.getContextPath() + "/webadmin/login");
		return "logout";
	}
	
	
	//编写api返回字符串或者json的几种方式使用ajax获取
	
	//单一接口适合使用
	@RequestMapping("/webapi")
	public void webInfo(HttpServletRequest request, HttpServletResponse response) throws IOException {
		
		PrintWriter out = response.getWriter();
		out.print("abc");
		
	}
	
	//单一接口适合使用
	@RequestMapping("/webapi2")
	@ResponseBody
	public String webInfo2() {
		return "index";
	}
	
	//复杂的返回逻辑适合使用，操作较为灵活
	@RequestMapping("/webapi3")
	public String webInfo3(HttpServletResponse response) throws IOException {
		PrintWriter out = response.getWriter();
		out.print("abc");
		return null;
	}
	
	//另外方式ajax请求返回一个部分视图，添加到页面中
	// 设置一个纯净单一的jsp页面供返回或跳转使用
	
}
