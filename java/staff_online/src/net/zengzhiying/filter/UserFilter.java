package net.zengzhiying.filter;

import java.io.IOException;
import javax.servlet.Filter;
import javax.servlet.FilterChain;
import javax.servlet.FilterConfig;
import javax.servlet.ServletException;
import javax.servlet.ServletRequest;
import javax.servlet.ServletResponse;
import javax.servlet.annotation.WebFilter;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

/**
 * 用户登录权限限制过滤器
 */
@WebFilter("/UserFilter")
public class UserFilter implements Filter {

    /**
     * Default constructor. 
     */
    public UserFilter() {
        // TODO Auto-generated constructor stub
    }

	/**
	 * @see Filter#destroy()
	 */
	public void destroy() {
		System.out.println("注销User Filter....");
	}

	/**
	 * @see Filter#doFilter(ServletRequest, ServletResponse, FilterChain)
	 */
	public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain) throws IOException, ServletException {
		HttpServletRequest req = (HttpServletRequest) request;
		HttpServletResponse rep = (HttpServletResponse) response;
		String servletUrl = req.getServletPath();
		//登录限制
		//获取通用session
		String sessionStr = (String) req.getSession().getAttribute("user");
		//获取超级管理员权限位 普通管理员没有
		//String superStr = (String) req.getSession().getAttribute("super_type");
		// 限制路由
		if(!"/webadmin/login".equals(servletUrl) && !"/webadmin/logout".equals(servletUrl) && !"/webadmin/error.htm".equals(servletUrl)) {
			if(sessionStr == null || sessionStr.equals("")) {
				rep.sendRedirect(req.getContextPath() + "/webadmin/login?redirect=false");
			} else {
				//放行 由控制器内部控制跳转
				chain.doFilter(request, response);
			}
		} else {
			chain.doFilter(request, response);
		}
	}

	/**
	 * @see Filter#init(FilterConfig)
	 */
	public void init(FilterConfig fConfig) throws ServletException {
		System.out.println("初始化用户登录Filter...");
	}

}
