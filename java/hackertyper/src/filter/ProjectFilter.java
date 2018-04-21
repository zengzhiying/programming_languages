package filter;

import java.io.IOException;
import javax.servlet.Filter;
import javax.servlet.FilterChain;
import javax.servlet.FilterConfig;
import javax.servlet.ServletException;
import javax.servlet.ServletRequest;
import javax.servlet.ServletResponse;
import javax.servlet.annotation.WebFilter;
import javax.servlet.http.HttpServletRequest;

/**
 * Servlet Filter implementation class ProjectFilter
 */
@WebFilter(filterName="ProjectFilter", value={"/*"})
public class ProjectFilter implements Filter {

    /**
     * Default constructor. 
     */
    public ProjectFilter() {
        // TODO Auto-generated constructor stub
    }

	/**
	 * @see Filter#destroy()
	 */
	public void destroy() {
		// TODO Auto-generated method stub
	}

	/**
	 * @see Filter#doFilter(ServletRequest, ServletResponse, FilterChain)
	 */
	public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain) throws IOException, ServletException {
		HttpServletRequest httpreq = (HttpServletRequest) request;
		String url = httpreq.getScheme() + "://" + httpreq.getServerName() + ":" + httpreq.getServerPort() + httpreq.getServletPath();
		
		if(url.indexOf(".css") != -1 || url.indexOf(".js") != -1 || url.indexOf(".txt") != -1) {
			System.out.println("直接放行");
			chain.doFilter(request, response);
		} else {
			System.out.println("start filter");
			//编码处理
			request.setCharacterEncoding("utf-8");
			response.setCharacterEncoding("utf-8");
			response.setContentType("text/html; charset=utf-8");
			chain.doFilter(request, response);
		}
		
	}

	/**
	 * @see Filter#init(FilterConfig)
	 */
	public void init(FilterConfig fConfig) throws ServletException {
		// TODO Auto-generated method stub
	}

}
