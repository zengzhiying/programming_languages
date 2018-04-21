package net.zengzhiying.controller;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.RequestMapping;

/**
 * 测试模板控制器
 * @author zengzhiying
 *
 */
@Controller
@RequestMapping("/temp")
public class TempController {
	
	@RequestMapping({"/form","/index"})
	public String index() {
		return "temp/forms";
	}
	
	@RequestMapping({"/gallery"})
	public String gallery() {
		return "temp/gallery";
	}
	
}
