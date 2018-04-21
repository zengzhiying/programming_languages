package net.zengzhiying.filter;

import java.io.IOException;

import javax.servlet.Filter;
import javax.servlet.FilterChain;
import javax.servlet.FilterConfig;
import javax.servlet.ServletException;
import javax.servlet.ServletRequest;
import javax.servlet.ServletResponse;
import javax.servlet.http.HttpServletRequest;

/**
 * 全局过滤器，进行必要的初始化操作，如页面编码转换操作等
 * 
 */
public class GeneralFilter implements Filter {

	@Override
	public void destroy() {
		System.out.println("General Filter destroy...");
	}

	@Override
	public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain)
			throws IOException, ServletException {
		//获取访问的url
		HttpServletRequest http = (HttpServletRequest) request;
		String servletUrl = http.getServletPath();
		String projectPath = http.getContextPath();
		String url = projectPath + servletUrl;
		if(!url.contains(".jpg") && !url.contains(".js") && !url.contains(".css") && !url.contains(".png") && !url.contains(".ico")) {
			request.setCharacterEncoding("utf-8");
			response.setCharacterEncoding("utf-8");
			response.setContentType("text/html; charset=utf-8");
		}
		
		chain.doFilter(request, response);
	}

	@Override
	public void init(FilterConfig arg0) throws ServletException {
		System.out.println("General Filter start...");
	}

}
