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

@Controller("StaffController")
@RequestMapping(value={"/webadmin"})
public class StaffController {
	
	@RequestMapping("/stafflist")
	public String staffList(HttpServletRequest request) {
		//如果是超级管理员可以查看所有员工
		if(request.getSession().getAttribute("super_type") != null) {
			StaffService ss = new StaffService();
			List<Object> userList = ss.staffList(3);
			List<UserToPart> upList = new ArrayList<UserToPart>();
			//列出所有部门数据 便于搜索
			PartmentService ps = new PartmentService();
			List<Object> pmList = ps.partmentList();
			request.setAttribute("pmList", pmList);
			// 查看所有员工
			//组装数据获取部门
			if(request.getParameter("partment_id") == null || request.getParameter("partment_id").equals("")) {
				//查看所有员工
				for(Object user:userList) {
					User u = (User) user;
					if(request.getParameter("staff_name") != null && !request.getParameter("staff_name").equals("")) {
						//按姓名模糊搜索
						String staff_name = request.getParameter("staff_name");
						if(u.getUser_name().contains(staff_name)) {
							Partment pm = ss.getPartment(u.getDepartment_id());
							UserToPart up = new UserToPart(u, pm);
							upList.add(up);
						}
					} else {
						Partment pm = ss.getPartment(u.getDepartment_id());
						UserToPart up = new UserToPart(u, pm);
						upList.add(up);
					}
					
				}
				request.setAttribute("staffList", upList);
			} else {
				//查看搜索的员工
				String partment_id = request.getParameter("partment_id");
				for(Object user:userList) {
					User u = (User) user;
					if(partment_id.equals(String.valueOf(u.getDepartment_id()))) {
						//只显示有对应部门的数据
						Partment pm = ss.getPartment(u.getDepartment_id());
						UserToPart up = new UserToPart(u, pm);
						upList.add(up);
					}
				}
				request.setAttribute("staffList", upList);
			}
			
		} else {
			// 是部门经理只查看本部门员工
			StaffService ss = new StaffService();
			List<UserToPart> upList = new ArrayList<UserToPart>();
			//由session查询对应的部门id
			if(request.getSession().getAttribute("user") != null) {
				String username = (String) request.getSession().getAttribute("user");
				//先根据username返回部门id
				int partment_id = ss.getPartmentId(username);
				// 只有一个部门，节省资源只查询一次
				Partment pm = ss.getPartment(partment_id);
				//查询某个部门id下对应的员工数据
				List<Object> userList = ss.staffList(3);
				//对比符合条件的员工数据
				for(Object user:userList) {
					User u = (User) user;
					if(u.getDepartment_id() == partment_id) {
						UserToPart up = new UserToPart(u, pm);
						upList.add(up);
					}
				}
				request.setAttribute("staffList", upList);
			} else {
				//直接退出
				return "error/superError";
			}
		}
		
		return "staff/index";
	}
	
	@RequestMapping({"/staffadd","/staffedit"})
	public String staffEdit(@RequestParam(value="user_id",required=false) String user_id,@RequestParam(value="action",required=false) String action, HttpServletRequest request,HttpServletResponse response) throws IOException {
		
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
				//默认员工审批状态为0
				int is_approve = 0;
				if(request.getSession().getAttribute("super_type") != null) {
					//超级管理员 直接转正
					is_approve = 1;
				}
				// 新增 start
				if(request.getSession().getAttribute("super_type") == null) {
					//普通管理员只能新增自己部门的员工
					String manage_name = (String) request.getSession().getAttribute("user");
					//重新设置partment_id的值为自己的员工
					partment_id = String.valueOf(ss.getPartmentId(manage_name));
					out.print(ss.addPartment(user_name, gender, age, partment_id, contact, user_describe, is_approve));
					
					// 后台写日志操作
					LoggerService.addLogs("添加", "管理员" + request.getSession().getAttribute("user") + "添加了员工" + user_name);
				} else {
					out.print(ss.addPartment(user_name, gender, age, partment_id, contact, user_describe, is_approve));
					
					// 后台写日志操作
					LoggerService.addLogs("添加", "管理员" + request.getSession().getAttribute("user") + "添加了用户" + user_name);
				}
				
				// 新增 end
				return null;
				
				
			}
			return "staff/edit";
		} else {
			// user_id 验证过滤
			// 参数获取
			String user_name = request.getParameter("user_name");
			String gender = request.getParameter("gender");
			String age = request.getParameter("age");
			String partment_id = request.getParameter("partment_id");
			String contact = request.getParameter("contact");
			String user_describe = request.getParameter("user_describe");
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
					LoggerService.addLogs("删除", "管理员" + request.getSession().getAttribute("user") + "删除的编号为" + user_id + "的用户");
				} else if(action != null && action.equals("edit")) {
					//System.out.println("执行到这....");
					// 修改start
					if(request.getSession().getAttribute("super_type") == null) {
						//普通管理员只能修改自己部门的员工
						String manage_name = (String) request.getSession().getAttribute("user");
						//重新设置partment_id的值为自己的员工
						partment_id = String.valueOf(ss.getPartmentId(manage_name));
						out.print(ss.addPartment(user_name, gender, age, partment_id, contact, user_describe, is_approve, user_id));
						
						// 后台写日志操作
						LoggerService.addLogs("修改", "管理员" + request.getSession().getAttribute("user") + "对用户" + user_name + "进行了修改");
					} else {
						out.print(ss.addPartment(user_name, gender, age, partment_id, contact, user_describe, is_approve, user_id));
						
						// 后台写日志操作
						LoggerService.addLogs("修改", "管理员" + request.getSession().getAttribute("user") + "对用户" + user_name + "进行了修改");
					}
					
					// 修改 end
				}
				return null;
			} else {
				// 修改界面
				User u = ss.getUser(user_id);
				if(u == null) {
					return "error/paramError";
				}
				request.setAttribute("user", u);
				return "staff/edit";
			}
		}
	}
	
	/**
	 * 员工审批 指定必须是post请求和user_id字段 否则会报错
	 * @param request
	 * @return
	 * @throws IOException 
	 */
	@RequestMapping(value="/approve",method=RequestMethod.POST)
	public String approve(HttpServletRequest request,HttpServletResponse response,@RequestParam(value="user_id",required=true) String user_id) throws IOException {
		if(request.getSession().getAttribute("super_type") != null) {
			PrintWriter out = response.getWriter();
			//执行操作
			StaffService ss = new StaffService();
			//校验员工真实身份
			if(user_id != null && !user_id.equals("")) {
				//查询员工存在性
				User u = ss.getUser(user_id);
				if(u != null && u.getAccess_type() == 3 && u.getIs_approve() == 0) {
					// 真正修改审批状态
					out.print(ss.userApprove(Integer.valueOf(user_id)));
					
					// 后台写日志操作
					LoggerService.addLogs("审批", "管理员" + request.getSession().getAttribute("user") + "对编号为" + user_id + "的用户通过了审批");
				} else {
					out.print("error");
				}
			} else {
				out.print("error");
			}
		}
		return null;
	}
	
	
	
}
