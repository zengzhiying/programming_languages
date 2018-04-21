package net.zengzhiying.controller;

import java.io.IOException;
import java.io.Writer;

import javax.servlet.http.HttpServletRequest;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;
import org.springframework.web.bind.annotation.RequestParam;

/**
 * 测试页面
 * @author Administrator
 *
 */

@Controller("Course")
@RequestMapping({"/Course"})
public class CourseController {

	//只处理get请求
	@RequestMapping(value={"/"},method=RequestMethod.GET)
	public String Views(@RequestParam(value="courseId",required=false) Integer courseId) {
		
		System.out.println(courseId);
		return "temp/index";
	}
	
	@RequestMapping(value={"/views2/{courseId}"})
	public String Views2(@PathVariable("courseId") Integer courseId) {
		
		//System.out.println(courseId);
		
		return "temp/views";
	}
	
	@RequestMapping(value="/views3",method=RequestMethod.POST)
	public String views3(HttpServletRequest request) {
		System.out.println(request.getParameter("courseId"));
		return "temp/views";
	}
	
	@RequestMapping("/views4")
	public void views4(@RequestBody(required=true) String body, Writer writer) {
		//System.out.println(body);
		try {
			writer.write(body);
		} catch (IOException e) {
			// TODO Auto-generated catch block
			e.printStackTrace();
		}
		//return "views";
	}
	
	
	@RequestMapping("/editpassword")
	public String editpassword() {
		return "temp/editpassword";
	}
	
	
	@RequestMapping("/shenpi")
	public String shenpi() {
		return "temp/shenpi";
	}
	
	@RequestMapping("/count")
	public String count() {
		return "temp/count";
	}
	
	@RequestMapping("/login")
	public String login() {
		return "temp/login";
	}
	
	@RequestMapping("/logs")
	public String logs() {
		return "temp/logs";
	}
	
	
	
}
