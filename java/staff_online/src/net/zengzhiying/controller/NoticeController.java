package net.zengzhiying.controller;

import java.io.IOException;
import java.io.PrintWriter;
import java.util.List;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.RequestMapping;

import net.zengzhiying.beans.Notice;
import net.zengzhiying.service.NoticeService;

/**
 * 公告管理控制器
 * @author zengzhiying
 *
 */

@Controller("NoticeController")
@RequestMapping(value={"/webadmin"})
public class NoticeController {
	
	@RequestMapping("/notice.htm")
	public String noticeList(HttpServletRequest request) {
		List<Notice> ntList = new NoticeService().noticeList();
		
		request.setAttribute("pmAll", ntList);
		
		return "notice/index";
	}
	
	@RequestMapping(value={"/notice/add","/notice/edit"})
	public String noticeEdit(HttpServletRequest request, HttpServletResponse response) throws IOException {
		String nt_title = request.getParameter("nt_title");
		String nt_desc = request.getParameter("nt_desc");
		String nt_content = request.getParameter("nt_content");
		if(request.getSession().getAttribute("super_type") != null) {
			if(request.getMethod().equals("POST")) {
				//修改或添加处理
				PrintWriter out = response.getWriter();
				if(request.getParameter("nt_id") == null || request.getParameter("nt_id").equals("")) {
					out.print(new NoticeService().addNotice(nt_title,nt_desc,nt_content));
					return null;
				} else {
					String nt_id = request.getParameter("nt_id");
					out.print(new NoticeService().updateNoticeOne(Integer.valueOf(nt_id),nt_title,nt_desc,nt_content));
					return null;
				}
			} else {
				if(request.getParameter("nt_id") == null || request.getParameter("nt_id").equals("")) {
					//新增界面
					return "notice/edit";
				} else {
					String nt_id = request.getParameter("nt_id");
					System.out.println("执行到这...");
					Notice nt = new NoticeService().noticeOne(Integer.valueOf(nt_id));
					request.setAttribute("nt", nt);
					return "notice/edit";
				}
			}
		} else {
			return "error/superError";
		}
	}
	
	/**
	 * 删除
	 * @param request
	 * @return
	 * @throws IOException 
	 */
	@RequestMapping("/notice/del.htm")
	public String deleteNotice(HttpServletRequest request, HttpServletResponse response) throws IOException {
		if(request.getSession().getAttribute("super_type") != null && request.getMethod().equals("POST")) {
			String nt_id = request.getParameter("nt_id");
			PrintWriter out = response.getWriter();
			out.print(new NoticeService().delete(Integer.valueOf(nt_id)));
			return null;
		} else {
			return "error/superError";
		}
	}
	
}
