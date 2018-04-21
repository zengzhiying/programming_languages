package net.zengzhiying.controller;

import java.io.IOException;
import java.io.PrintWriter;
import java.io.Writer;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestMethod;
import org.springframework.web.bind.annotation.RequestParam;

import net.zengzhiying.beans.PartToNumber;
import net.zengzhiying.beans.Partment;
import net.zengzhiying.beans.User;
import net.zengzhiying.beans.UserToPart;
import net.zengzhiying.service.LoggerService;
import net.zengzhiying.service.PartmentService;
import net.zengzhiying.service.StaffService;

/**
 * 员工管理控制器
 * @author zengzhiying
 *
 */

@Controller("PartmentManageController")
@RequestMapping(value={"/webadmin"})
public class PartmentManageController {
	
	@RequestMapping("/partment_manager")
	public String staffList(HttpServletRequest request) {
		//权限控制 start
		if(request.getSession().getAttribute("super_type") == null) {
			return "error/superError";
		}
		// 权限控制 end
		StaffService ss = new StaffService();
		PartmentService ps = new PartmentService();
		List<Object> userList = ss.staffList(2);
		List<PartToNumber> upList = new ArrayList<PartToNumber>();
		//组装数据获取部门名称和人数
		for(Object user:userList) {
			User u = (User) user;
			Partment pm = ss.getPartment(u.getDepartment_id());
			UserToPart up = new UserToPart(u, pm);
			PartToNumber pn = new PartToNumber();
			//upList.add(up);
			// 计算部门的人数
			pn.setUtp(up);
			pn.setNumber(ps.staffCount(u.getDepartment_id()));
			upList.add(pn);
		}
		
		
		request.setAttribute("staffList", upList);
		return "partment_manage/index";
	}
	
	@RequestMapping({"/partment_manage/add","/partment_manage/edit"})
	public String staffEdit(@RequestParam(value="user_id",required=false) String user_id,@RequestParam(value="action",required=false) String action, HttpServletRequest request,HttpServletResponse response) throws IOException {
		//权限控制 start
		if(request.getSession().getAttribute("super_type") == null) {
			return "error/superError";
		}
		// 权限控制 end
		
		//列出所有部门数据 便于后来使用
		PartmentService ps = new PartmentService();
		List<Object> pmList = ps.partmentList();
		request.setAttribute("pmList", pmList);
		PrintWriter out = response.getWriter();
		//实例化员工服务类
		StaffService ss = new StaffService();
		
		if(user_id == null || user_id.equals("")) {
			// 新增界面
			if(request.getMethod().equals("POST")) {
//				System.out.println("执行到这....");
				// 参数获取
				String user_name = request.getParameter("user_name");
				String gender = request.getParameter("gender");
				String age = request.getParameter("age");
				String partment_id = request.getParameter("partment_id");
				String contact = request.getParameter("contact");
				String user_describe = request.getParameter("user_describe");
				String username = request.getParameter("username");
				String password = request.getParameter("password");
				//默认员工审批状态为0
				int is_approve = 1;
				
				// 新增 start
				out.print(ss.addManagePartment(user_name, gender, age, partment_id, contact, user_describe, is_approve,username,password));
				// 新增 end
				// 后台写日志操作
				LoggerService.addLogs("添加", "管理员" + request.getSession().getAttribute("user") + "添加了部门经理" + user_name);
				return null;
				
				
			}
			return "partment_manage/edit";
		} else {
			// user_id 验证过滤
			// 参数获取
			String user_name = request.getParameter("user_name");
			String gender = request.getParameter("gender");
			String age = request.getParameter("age");
			String partment_id = request.getParameter("partment_id");
			String contact = request.getParameter("contact");
			String user_describe = request.getParameter("user_describe");
			String username = request.getParameter("username");
			String password = request.getParameter("password");
			//默认员工审批状态为0
			int is_approve = 0;
			if(request.getSession().getAttribute("super_type") != null) {
				//超级管理员 直接转正
				is_approve = 1;
			}
			if(request.getMethod().equals("POST")) {
				// 修改或者删除
				if(action != null && action.equals("delete")) {
					//删除 start
					out.print(ss.deleteUser(user_id));
					//删除 end
					// 后台写日志操作
					LoggerService.addLogs("删除", "管理员" + request.getSession().getAttribute("user") + "删除了编号为" + user_id + "的部门经理");
				} else if(action != null && action.equals("edit")) {
					//System.out.println("执行到这....");
					// 修改start
					out.print(ss.addManagePartment(user_name, gender, age, partment_id, contact, user_describe, is_approve, username,password,user_id));
					// 修改 end
					// 后台写日志操作
					LoggerService.addLogs("修改", "管理员" + request.getSession().getAttribute("user") + "对部门经理" + user_name + "进行了修改");
				}
				return null;
			} else {
				// 修改界面
				User u = ss.getUser(user_id);
				if(u == null) {
					return "error/paramError";
				}
				request.setAttribute("user", u);
				return "partment_manage/edit";
			}
		}
	}
	
	
	
}
