<%@ page language="java" contentType="text/html; charset=UTF-8"
    pageEncoding="UTF-8"%>
<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">
<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
<title>test1</title>
</head>
<body>
<%
String test1 = (String) request.getAttribute("test1");
out.print(test1);

//out.print(request.getParameter("abc"));
%>
</body>
</html>