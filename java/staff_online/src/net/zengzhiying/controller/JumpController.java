package net.zengzhiying.controller;

import java.io.IOException;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.RequestMapping;

/**
 * 默认跳转控制器
 * @author zengzhiying
 *
 */

@Controller("JumpController")
@RequestMapping(value={"","/"})
public class JumpController {

	@RequestMapping(value={"","/"})
	public String index(HttpServletRequest request, HttpServletResponse response) {
		try {
			response.sendRedirect(request.getContextPath() + "/webadmin/");
		} catch (IOException e) {
			e.printStackTrace();
		}
		return null;
	}
}
