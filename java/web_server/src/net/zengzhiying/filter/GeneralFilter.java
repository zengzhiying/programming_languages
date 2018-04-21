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
 * Servlet Filter implementation class GlobalFilter
 * 全局过滤器，转换所有请求的编码
 */

public class GeneralFilter implements Filter {

    /**
     * Default constructor. 
     */
    public GeneralFilter() {
        // TODO Auto-generated constructor stub
    }

	/**
	 * @see Filter#destroy()
	 */
	public void destroy() {
		System.out.println("General filter destroy");
	}

	/**
	 * @see Filter#doFilter(ServletRequest, ServletResponse, FilterChain)
	 */
	public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain) throws IOException, ServletException {
		
		//获取访问的url
		HttpServletRequest http = (HttpServletRequest) request;
		String url = http.getContextPath() + http.getServletPath();
		
		if(!url.contains(".jpg") && !url.contains(".js") && !url.contains(".css") && !url.contains(".png") && !url.contains(".ico")) {
			request.setCharacterEncoding("utf-8");
			response.setCharacterEncoding("utf-8");
			response.setContentType("text/html; charset=utf-8");
		}
		chain.doFilter(request, response);
	}

	/**
	 * @see Filter#init(FilterConfig)
	 */
	public void init(FilterConfig fConfig) throws ServletException {
		System.out.println("General filter start");
	}

}
