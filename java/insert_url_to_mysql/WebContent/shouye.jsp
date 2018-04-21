<%@ page language="java" contentType="text/html; charset=UTF-8"
    pageEncoding="UTF-8"%>
<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">
<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
<title>首页</title>
</head>
<body>
<h3>网址管理</h3>

<form action="insert_url/" method="post">
	名称：<input type="text" name="title" required="required" />
	<br /><br />
	网址：<input type="text" name="url" required="required" />
	<br /><br />
	分类：<input type="text" name="classify" value="时事论坛" required="required" />
	<br /><br />
	<%
	Cookie[] cookies = request.getCookies();
	String num = "";
	if(cookies != null && cookies.length > 0) {
		for(Cookie c:cookies) {
			if(c.getName().equals("num")) {
				num = c.getValue();
			}
		}
	}
	%>
	排序：<input type="text" name="sort" value="<%=num %>" required="required" />
	<br /><br />
	<input type="submit" />
</form>
</body>
</html>