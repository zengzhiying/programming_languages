package net.zengzhiying.controllers;

import java.io.IOException;
import java.io.PrintWriter;

import javax.servlet.ServletException;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

import org.eclipse.jetty.server.Request;
import org.eclipse.jetty.server.handler.AbstractHandler;

public class TestController extends AbstractHandler {

	@Override
	public void handle(String target, Request baseRequest, HttpServletRequest request, HttpServletResponse response)
			throws IOException, ServletException {
		System.out.println(target);
		response.setContentType("text/html; charset=utf-8");
		request.setCharacterEncoding("utf-8");
		response.setStatus(HttpServletResponse.SC_OK);
		baseRequest.setHandled(true);
		PrintWriter out = response.getWriter();
		if(target.equals("/favicon.ico")) {
			System.out.println("1");
			out.println("404");
		} else {
			System.out.println("2");
			out.print("<h3>hello jetty!</h3>");
			if(request.getParameter("name") != null) {
				out.print(request.getParameter("name"));
			}
		}
	}

}
