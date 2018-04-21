package servlet;

import java.io.IOException;
import java.io.PrintWriter;

import javax.servlet.ServletException;
import javax.servlet.annotation.WebServlet;
import javax.servlet.http.Cookie;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

import entity.Website;
import services.DbOperation;
import services.verify.DataVerify;

/**
 * Servlet implementation class LinkAdmin
 */
@WebServlet(urlPatterns={"/insert_url","/insert_url/"})
public class LinkInsert extends HttpServlet {
	private static final long serialVersionUID = 1L;
       
    /**
     * @see HttpServlet#HttpServlet()
     */
    public LinkInsert() {
        super();
        // TODO Auto-generated constructor stub
    }

	/**
	 * @see HttpServlet#doGet(HttpServletRequest request, HttpServletResponse response)
	 */
	protected void doGet(HttpServletRequest request, HttpServletResponse response) throws ServletException, IOException {
		doPost(request,response);
	}

	/**
	 * @see HttpServlet#doPost(HttpServletRequest request, HttpServletResponse response)
	 */
	protected void doPost(HttpServletRequest request, HttpServletResponse response) throws ServletException, IOException {
		//提交处理 编码统一处理
		request.setCharacterEncoding("utf-8");
		response.setCharacterEncoding("utf-8");
		response.setContentType("text/html; charset=utf-8");
		PrintWriter out = response.getWriter();
		String[] perame = {"title","url","classify","sort"};
		int j = 0;
		for(int i = 0;i < perame.length;i++) {
			if(request.getParameter(perame[i]) == null || request.getParameter(perame[i]).equals("")) {
				j++;
			}
		}
		if(j == 0) {
			String title = request.getParameter("title");
			String link = request.getParameter("url");
			String classify = request.getParameter("classify");
			if(DataVerify.isPosInt(request.getParameter("sort"))) {
				int sort = Integer.valueOf(request.getParameter("sort"));
//				out.print(title);
//				out.print(link);
//				out.print(classify);
//				out.print(sort);
				//设置cookie
				Cookie number = new Cookie("num",String.valueOf(sort+1));
				number.setPath("/");
				number.setMaxAge(24*60*60);
				//推送cookie
				response.addCookie(number);
				Website ws = new Website();
				ws.setTitle(title);
				ws.setLink(link);
				ws.setClassify(classify);
				ws.setSort(sort);
				DbOperation logic = new DbOperation();
				if(logic.addData(ws)) {
					out.print("<h2>恭喜你！插入成功！即将返回...</h2>");
					out.print("<script>setTimeout(\"location.href='../'\",3000);</script>");
				} else {
					out.print("<h2>插入失败！即将返回...</h2>");
					out.print("<script>setTimeout(\"location.href='../'\",3000);</script>");
				}
			} else {
				request.setAttribute("state", "排序必须输入正整数");
				request.getRequestDispatcher("/error.jsp").forward(request, response);
			}
			
			
		} else {
			System.out.print("test");
			request.setAttribute("state", "页面错误");
			request.getRequestDispatcher("/error.jsp").forward(request, response);
		}
		
	}

}
