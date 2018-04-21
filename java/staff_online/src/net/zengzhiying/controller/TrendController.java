package net.zengzhiying.controller;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.RequestMapping;

/**
 * 发展趋势Controller
 * @author zengzhiying
 *
 */
@Controller("TrendController")
@RequestMapping("/webadmin/trend")
public class TrendController {
	
	@RequestMapping("/income.show")
	public String showIncome() {
		return "thrend/income";
	}
	
	@RequestMapping("/income.view")
	public String showIncome1() {
		return "thrend/income_html";
	}
	
	@RequestMapping("/produce.show")
	public String shouProduce() {
		return "thrend/income_html2";
	}
	
}
