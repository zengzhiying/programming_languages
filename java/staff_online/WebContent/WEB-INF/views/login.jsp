<%@ page language="java" contentType="text/html; charset=UTF-8"
    pageEncoding="UTF-8"%>
<%
	String pagePath = request.getContextPath(); //获取页面通用的path上下文
%>
<!DOCTYPE html>
<html>
<head lang="en">
  <meta charset="UTF-8">
  <title>Login Page | Amaze UI Example</title>
  <meta http-equiv="X-UA-Compatible" content="IE=edge">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="format-detection" content="telephone=no">
  <meta name="renderer" content="webkit">
  <meta http-equiv="Cache-Control" content="no-siteapp" />
  <link rel="alternate icon" type="image/png" href="<%=pagePath %>/static/assets/i/favicon.png">
  <link rel="stylesheet" href="<%=pagePath %>/static/assets/css/amazeui.min.css"/>
  <style>
    .header {
      text-align: center;
    }
    .header h1 {
      font-size: 200%;
      color: #333;
      margin-top: 30px;
    }
    .header p {
      font-size: 14px;
    }
  </style>
</head>
<body>
<div class="header">
  <div class="am-g">
    <h1>员工上线后台管理系统登录</h1>
    <p>web<br/>xxxx.com</p>
  </div>
  <hr />
</div>
<div class="am-g">
  <div class="am-u-lg-6 am-u-md-8 am-u-sm-centered">
    <h3>请登录</h3>
    <hr>
    <br>

    <form method="post" class="am-form" action="<%=pagePath %>/webadmin/login">
      <label for="username">用户名:</label>
      <input type="text" name="username" id="username" value="" required="required">
      <br>
      <label for="password">密码:</label>
      <input type="password" name="password" id="password" value="" required="required">
      <br>
      <!-- 
      <label for="remember-me">
        <input id="remember-me" type="checkbox">
        记住密码
      </label> -->
      
      <br />
      <div class="am-cf">
        <input type="submit" name="" value="登  录" class="am-btn am-btn-primary am-btn-sm am-fl">
        <!-- 
        <input type="submit" name="" value="忘记密码 ^_^? " class="am-btn am-btn-default am-btn-sm am-fr">
         -->
      </div>
    </form>
    <hr>
    <p>© 2016 员工上线后台管理系统登录, Inc.</p>
  </div>
</div>
</body>
</html>
