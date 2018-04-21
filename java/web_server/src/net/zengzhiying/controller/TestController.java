package net.zengzhiying.controller;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.servlet.ModelAndView;

import net.zengzhiying.beans.User;
import net.zengzhiying.service.CalcThread;

/**
 * Spring 测试项目
 * @author spring
 *
 */
//可以直接写@Controller也可以写成下面的形式
@Controller("TestController")
//外层路由，不写默认直接访问下层方法
@RequestMapping({"/Test",""})
public class TestController {
	
	//方式1
	@RequestMapping({"/test1"})
	public ModelAndView test1() {
		String test1 = "test1";
		ModelAndView mv = new ModelAndView();
		mv.addObject("test1", test1);
		mv.setViewName("test1");
		return mv;
	}
	
	
	//方式2
	@RequestMapping("/test2")
	public String tpl2(HttpServletRequest request, HttpServletResponse response) {
		
		request.setAttribute("test2", "test2");
		return "test2";
		
	}
	
	@RequestMapping("/")
	public String index() {
		//这里多级目录可以写为return "pages/index";
		return "index";
	}
	
	//方法传参，参数不存在不会出错，为null
	@RequestMapping("/test3")
	public String test3(Integer testID) {
		System.out.println(testID);
		return "test";
	}
	
	//定义只能处理的请求类型，并对请求字符串和变量字符串分别处理，required指定参数不存在时不会报错，必须指定，否则不存在就会报错
	@RequestMapping(value="/test4",method=RequestMethod.GET)
	public String test4(@RequestParam(value="testId",required=false) Integer tests) {
		System.out.println(tests);
		return "test";
	}
	
	//现代风格的url传参，spring自动完成变量的绑定和类型的转换，不用手动分离正则表达式了
	@RequestMapping(value="/test5/{id}",method=RequestMethod.GET)
	public String test5(@PathVariable("id") Integer testid) {
		System.out.println(testid);
		return "test";
	}
	
	
	//传统方式获取url参数
	@RequestMapping(value="/test6",method=RequestMethod.GET)
	public String test6(HttpServletRequest req) {
		System.out.println(req.getParameter("id"));
		//获取请求的方法类型，判断是get还是post请求
		System.out.println(req.getMethod());
		return "test";
	}
	
	//表单请求直接转换为对象，表单name和对象属性要完全一致
	@RequestMapping(value="/test7",method=RequestMethod.POST)
	public String test7(User user) {
		System.out.println(user.getUsername() + " " + user.getPassword());
		return "test";
	}
	
	//线程异步计算
	@RequestMapping(value="/savefile", method=RequestMethod.GET)
	public String savefile(@RequestParam(value="text",required=false) String text, HttpServletRequest req) {
		if(text == null || text.equals("")) {
			req.setAttribute("status", "内容不能为空！");
		} else {
			CalcThread ct = new CalcThread(text);
			ct.start();
			System.out.println("线程启动....");
			req.setAttribute("status", "异步计算线程已经启动，请稍后查看结果，您可以关闭浏览器窗口");
		}
		return "savefile";
	}
	
}
