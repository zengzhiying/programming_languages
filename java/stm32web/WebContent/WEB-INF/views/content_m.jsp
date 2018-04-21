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
  <div class="am-g">
  	<div class="am-u-sm-12" style="margin-top:6%; text-align:center;">
  		<h1>Stm32传感器参数显示界面</h1>
  	</div>
  </div>
  <br />
  <div class="am-g" style="font-size:23px;">
    <div class="am-u-sm-10" style="margin-left:16%;">
    	<p>实时温度：<span id="view-num"></span></p>
    	<p>实时PM2.5：<span id="view-num2"></span></p>
    </div>
  </div>
  <br />
  <div class="am-g">
    <div class="am-u-sm-2"></div>
    <div class="am-u-sm-8">
    <button type="button" class="am-btn am-btn-secondary" id="numAction">&nbsp;&nbsp;&nbsp;&nbsp;Strat&nbsp;&nbsp;&nbsp;&nbsp;</button>
    &nbsp;&nbsp;&nbsp;
    <button type="button" class="am-btn am-btn-secondary" onclick="location.href='<%=pagePath %>/logout.html?type=mobile'">&nbsp;&nbsp;&nbsp;&nbsp;退出&nbsp;&nbsp;&nbsp;&nbsp;</button>
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
  $(document).ready(function() {
	  
	  var stm_data;
	  
	  document.getElementById('view-num').innerHTML
      = 'click Start action!';
	  
	  $("#numAction").click(function() {
		  loopGet();
	  });
	  
	  function getServer() {
		//请求开始
		  $.get("<%=pagePath %>/get.htm",{},function(data) {
			  //处理获取的字符串并推送
			  var old_data = stm_data;
			  stm_data = data;
			  if(old_data != stm_data) {
				  var arr_data = stm_data.split(' ');
		    	  $("#view-num").hide();
		    	  $("#view-num2").hide();
				  $("#view-num").html(arr_data[0] + " ℃");
				  $("#view-num2").html(arr_data[1] + " μg/m³");
				  $("#view-num").fadeIn();
				  $("#view-num2").fadeIn();
			  }
		  });
	  }
	  
	  function loopGet() {
		  //先执行一次请求获得最初变量
		  $.get("<%=pagePath %>/get.htm",{},function(data) {
			  stm_data = data;
			  var arr_data = stm_data.split(' ');
	    	  $("#view-num").hide();
	    	  $("#view-num2").hide();
			  $("#view-num").html(arr_data[0] + " ℃");
			  $("#view-num2").html(arr_data[1] + " μg/m³");
			  $("#view-num").fadeIn();
			  $("#view-num2").fadeIn();
		  });
		  setInterval(function() { getServer() }, 3000);
	  }
	  
  });
  </script>
</body>
</html>