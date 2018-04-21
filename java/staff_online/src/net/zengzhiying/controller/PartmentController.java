package net.zengzhiying.controller;

import java.io.IOException;
import java.io.PrintWriter;
import java.util.List;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;

import net.zengzhiying.beans.Partment;
import net.zengzhiying.service.LoggerService;
import net.zengzhiying.service.PartmentService;

/**
 * 部门管理控制器
 * @author zengzhiying
 *
 */

@Controller("PartmentController")
@RequestMapping("/webadmin")
public class PartmentController {
	
	/**
	 * 部门列表
	 * @return
	 */
	@RequestMapping("/partment")
	public String partmentList(HttpServletRequest request) {
		//权限控制 start
		if(request.getSession().getAttribute("super_type") == null) {
			return "error/superError";
		}
		// 权限控制 end
		PartmentService ps = new PartmentService();
		List<Object> pmAll = ps.partmentList();
		request.setAttribute("pmAll", pmAll);
		return "partment/index";
	}
	
	@RequestMapping(value={"/partment/edit","/partment/add"})
	public String editPartment(HttpServletRequest request, HttpServletResponse response, @RequestParam(value="partment_id",required=false) String partment_id,@RequestParam(value="action",required=false) String action) throws IOException {
		//权限控制 start
		if(request.getSession().getAttribute("super_type") == null) {
			return "error/superError";
		}
		// 权限控制 end
		if(request.getMethod().equals("POST")) {
			//在里面获取对应属性
			String pm_name = request.getParameter("pm_name");
			String pm_describe = request.getParameter("pm_describe");
			String setup_time = request.getParameter("setup_time");
			String pm_id = request.getParameter("pm_id");
			//实例化必要对象
			PartmentService ps = new PartmentService();	//实例化service方法
			PrintWriter out = response.getWriter();	//创建输出
			
			//判断提交类型逻辑
			//处理修改或者新增或者删除
			if(action != null && action.equals("edit")) {
				//修改 start
				out.print(ps.addPartment(pm_name, pm_describe, setup_time, pm_id));
				
				// 后台写日志操作
				LoggerService.addLogs("修改", "管理员" + request.getSession().getAttribute("user") + "对部门" + pm_name + "进行了修改");
				return null;
				// 修改end
			}
			if(action != null && action.equals("delete")) {
				//删除 start
				out.print(ps.deletePartment(pm_id));
				
				// 后台写日志操作
				LoggerService.addLogs("删除", "管理员" + request.getSession().getAttribute("user") + "删除了编号为" + pm_id + "的部门");
				return null;
				//删除 end
			}
			
			//新增 start
			//全部交由service进行处理并直接将结果返回给前端
			out.print(ps.addPartment(pm_name,pm_describe,setup_time));
			// 后台写日志操作
			LoggerService.addLogs("添加", "管理员" + request.getSession().getAttribute("user") + "添加了部门：" + pm_name + "");
			return null;
			//新增 end
		}
		if(partment_id == null || partment_id.equals("")) {
			return "partment/edit";
		} else {
			// 修改界面 start
			PartmentService ps = new PartmentService();
			Partment pm = ps.editPartment(partment_id);
			if(pm == null) {
				return "error/paramError";
			}
			request.setAttribute("pm", pm);
			return "partment/edit";
			// 修改界面 end
		}
	}
	
}
