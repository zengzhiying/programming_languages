package net.zengzhiying.controller;

import java.util.List;

import javax.servlet.http.HttpServletRequest;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;

import net.zengzhiying.beans.CheckWork;
import net.zengzhiying.service.CheckWorkService;

/**
 * 员工考勤统计控制器
 * @author zengzhiying
 *
 */
@Controller("CheckWorkController")
@RequestMapping(value={"/webadmin"})
public class CheckWorkController {
	
	
	/**
	 * 员工考勤查看
	 * @return
	 */
	@RequestMapping("/checkwork.htm")
	public String staffCheckWork(@RequestParam(value="user_id",required=false) String user_id, HttpServletRequest request) {
		if(request.getParameter("time") == null || request.getParameter("time").equals("")) {
			//查询本月考勤记录
			CheckWorkService cws = new CheckWorkService();
			List<CheckWork> cwResult = cws.workData(Integer.valueOf(user_id), 1);
			//返回数据
			request.setAttribute("cwList", cwResult);
			request.setAttribute("status", "new");
			
		} else {
			//查询上月记录
			CheckWorkService cws = new CheckWorkService();
			List<CheckWork> cwResult = cws.workData(Integer.valueOf(user_id), 2);
			//返回数据
			request.setAttribute("cwList", cwResult);
			request.setAttribute("status", "old");
		}
		
		
		
		
		return "checkwork/index";
	}

}
