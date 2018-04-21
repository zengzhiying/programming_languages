<%@ page language="java" contentType="text/html; charset=UTF-8"
    pageEncoding="UTF-8"%>
<%
    String pagePath = request.getParameter("pagePath");
%>
<footer>
  <hr>
  <p class="am-padding-left">© 2016 员工上线管理系统, Inc.</p>
</footer>

<!--[if lt IE 9]>
<script src="http://libs.baidu.com/jquery/1.11.1/jquery.min.js"></script>
<script src="http://cdn.staticfile.org/modernizr/2.8.3/modernizr.js"></script>
<script src="<%=pagePath %>/static/assets/js/amazeui.ie8polyfill.min.js"></script>
<![endif]-->

<!--[if (gte IE 9)|!(IE)]><!-->
<script src="<%=pagePath %>/static/assets/js/jquery.min.js"></script>
<!--<![endif]-->
<script src="<%=pagePath %>/static/assets/js/amazeui.min.js"></script>
<script src="<%=pagePath %>/static/assets/js/app.js"></script>