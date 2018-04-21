package net.zengzhiying.controller;

import java.io.IOException;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;
import org.springframework.web.bind.annotation.RequestParam;

import net.zengzhiying.beans.Message;

@Controller
@RequestMapping(value="/")
public class StmWebController {
	
	@RequestMapping(value="/",method=RequestMethod.GET)
	public String stmWeb(@RequestParam(value="type",required=false) String type, HttpServletRequest req, HttpServletResponse rep) {

		if(req.getRequestURI().equals("/stm32web")) {
			try {
				if(type == null || !type.equals("mobile")) {
					rep.sendRedirect("/stm32web/");
				} else if(type.equals("mobile")) {
					rep.sendRedirect("/stm32web/?type=mobile");
				}				
			} catch (IOException e) {
				e.printStackTrace();
			}
			return null;
		}
		if(req.getSession().getAttribute("webuser") == null) {
			try {
				rep.sendRedirect(req.getContextPath() + "/login.html?type=" + type);
			} catch (IOException e) {
				e.printStackTrace();
			}
			return null;
		}
		if(type != null && type.equals("mobile")) {
			return "stmview_m";
		} else {
			return "stmview";
		}
	}
	
	@RequestMapping(value="/send", method=RequestMethod.GET)
	public String sendTest(@RequestParam(value="wd",required=false) String data, HttpServletRequest req) {
		if(data != null && !data.equals("")) {
			Message.setName("data");
			Message.setData(data);
		}
		
		return "send";
	}
	
	@RequestMapping(value="/get.htm")
	public String queryData(HttpServletRequest req) {
		req.setAttribute("data", Message.getData());
		return "get";
	}
	
	@RequestMapping(value="/content")
	public String contentView() {
		try {
			Thread.sleep(1000);
		} catch (InterruptedException e) {
			e.printStackTrace();
		}
		return "content";
	}
	
	@RequestMapping(value="/content_m")
	public String contentViewm() {
		try {
			Thread.sleep(1000);
		} catch (InterruptedException e) {
			e.printStackTrace();
		}
		return "content_m";
	}
	
	@RequestMapping(value="/login.html")
	public String loginView(HttpServletRequest req, HttpServletResponse rep,@RequestParam(value="type",required=false) String type) {
		String reqType = req.getMethod();
		if(reqType.equals("POST")) {
			if(req.getParameter("username") != null && req.getParameter("password") != null && req.getParameter("username").equals("zengzy") && req.getParameter("password").equals("123456")) {
				req.getSession().setAttribute("webuser", "zengzy");
//				System.out.println(req.getSession().getAttribute("user"));
//				System.out.println("登录成功！");
				//跳转
				try {
					if(type == null || !type.equals("mobile")) {
						rep.sendRedirect(req.getContextPath() + "/");
					} else {
						rep.sendRedirect(req.getContextPath() + "?type=mobile");
					}
					
				} catch (IOException e) {
					e.printStackTrace();
				}
			} else {
				System.out.println("登录失败！");
				try {
					rep.sendRedirect(req.getContextPath() + "/login.html");
				} catch (IOException e) {
					e.printStackTrace();
				}
			}
			return null;
		}
		//req.setAttribute("type", reqType);
		//System.out.println(req.getSession().getAttribute("abcd"));
		return "login";
	}
	
	@RequestMapping("/tests/abc")
	public String tests(HttpServletRequest req) {
		req.getSession().setAttribute("abcd", "abcds");
		return "tests";
	}
	
	@RequestMapping("/logout.html")
	public String logoutView(HttpServletRequest req, HttpServletResponse rep, @RequestParam(value="type") String type) {
		req.getSession().invalidate();
		try {
			rep.sendRedirect(req.getContextPath() + "/?type=" + type);
		} catch (IOException e) {
			e.printStackTrace();
		}
		return null;
	}
	
	
	
}
