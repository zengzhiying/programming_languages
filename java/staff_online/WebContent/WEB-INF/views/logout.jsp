<%@ page language="java" contentType="text/html; charset=UTF-8"
    pageEncoding="UTF-8"%>
<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">
<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
<title>正在退出...</title>
</head>
<body>
正在注销用户....
<script type="text/javascript">
setTimeout(function() {
	location.href="<%=request.getContextPath() %>/webadmin/login?status=logout";
},2000);
</script>
</body>
</html>