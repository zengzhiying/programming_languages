<%@ page language="java" contentType="text/html; charset=UTF-8"
    pageEncoding="UTF-8"%>
    
<%
	String pagePath = request.getContextPath();
%>
<!doctype html>
<html class="no-js">
<head>
  <meta charset="utf-8">
  <meta http-equiv="X-UA-Compatible" content="IE=edge">
  <meta name="description" content="">
  <meta name="keywords" content="">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>stm32传感器参数实时显示</title>

  <!-- Set render engine for 360 browser -->
  <meta name="renderer" content="webkit">

  <!-- No Baidu Siteapp-->
  <meta http-equiv="Cache-Control" content="no-siteapp"/>

  <link rel="icon" type="image/png" href="http://s.amazeui.org/assets/2.x/i/favicon.png">

  <!-- Add to homescreen for Chrome on Android -->
  <meta name="mobile-web-app-capable" content="yes">
  <link rel="icon" sizes="192x192" href="http://s.amazeui.org/assets/2.x/i/app-icon72x72@2x.png">

  <!-- Add to homescreen for Safari on iOS -->
  <meta name="apple-mobile-web-app-capable" content="yes">
  <meta name="apple-mobile-web-app-status-bar-style" content="black">
  <meta name="apple-mobile-web-app-title" content="Amaze UI"/>
  <link rel="apple-touch-icon-precomposed" href="http://s.amazeui.org/assets/2.x/i/app-icon72x72@2x.png">

  <!-- Tile icon for Win8 (144x144 + tile color) -->
  <meta name="msapplication-TileImage" content="http://s.amazeui.org/assets/2.x/i/app-icon72x72@2x.png">
  <meta name="msapplication-TileColor" content="#0e90d2">

  <link rel="stylesheet" href="http://cdn.amazeui.org/amazeui/2.6.2/css/amazeui.min.css">
</head>
<body>


  <!-- view start -->
  <div class="am-g" id="view-box" style="margin-top:10%;">
  	<div class="am-u-sm-9">
  		<form action="<%=pagePath %>/login.html" class="am-form" method="post">
		  <fieldset>
		    <legend>请登录</legend>
		    <div class="am-form-group">
		      <label for="doc-vld-name">用户名：</label>
		      <input type="text" name="username" id="doc-vld-name" class="am-form-field" required/>
		      <label for="doc-vld-name">密码：</label>
		      <input type="password" name="password" id="doc-vld-name" class="am-form-field" required/>
		      <input type="hidden" name="type" value="<%=request.getParameter("type") %>">
		      <br />
		      <button class="am-btn am-btn-secondary" style="width:128px;" type="submit">登录</button>
		    </div>
		  </fieldset>
		</form>
  	</div>
  </div>
  <!-- view end -->


  <!--[if (gte IE 9)|!(IE)]><!-->
  <script src="http://apps.bdimg.com/libs/jquery/2.1.4/jquery.min.js"></script>
  <!--<![endif]-->
  <!--[if lte IE 8 ]>
  <script src="http://libs.baidu.com/jquery/1.11.3/jquery.min.js"></script>
  <script src="http://cdn.staticfile.org/modernizr/2.8.3/modernizr.js"></script>
  <script src="assets/js/amazeui.ie8polyfill.min.js"></script>
  <![endif]-->
  <script src="http://cdn.amazeui.org/amazeui/2.6.2/js/amazeui.min.js"></script>

  <script type="text/javascript">
  //自动点击隐藏按钮，页面自动显示loading状态
  
  $(document).ready(function() {
	  
    });
    
  });
  </script>
</body>
</html>