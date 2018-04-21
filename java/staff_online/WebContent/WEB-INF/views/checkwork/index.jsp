<%@page import="java.text.DecimalFormat"%>
<%@page import="net.zengzhiying.tools.DataTypeConver"%>
<%@page import="net.zengzhiying.beans.CheckWork"%>
<%@page import="java.util.List"%>
<%@ page language="java" contentType="text/html; charset=UTF-8"
    pageEncoding="UTF-8"%>
<%
	String pagePath = request.getContextPath(); //获取页面通用的path上下文
%>
<!doctype html>
<html class="no-js">
<head>
  <meta charset="utf-8">
  <meta http-equiv="X-UA-Compatible" content="IE=edge">
  <title>考勤统计</title>
  <meta name="description" content="这是一个form页面">
  <meta name="keywords" content="form">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="renderer" content="webkit">
  <meta http-equiv="Cache-Control" content="no-siteapp" />
  
  
  <link rel="icon" type="image/png" href="<%=pagePath %>/static/assets/i/favicon.png">
  <link rel="apple-touch-icon-precomposed" href="<%=pagePath %>/static/assets/i/app-icon72x72@2x.png">
  <meta name="apple-mobile-web-app-title" content="Amaze UI" />
  <link rel="stylesheet" href="<%=pagePath %>/static/assets/css/amazeui.min.css"/>
  <link rel="stylesheet" href="<%=pagePath %>/static/assets/css/admin.css">
</head>
<body>


<!-- 公共 header头部通栏 start -->
<jsp:include page="/WEB-INF/views/public/header.jsp"></jsp:include>
<!-- 公共header头部通栏 end -->
<div class="am-cf admin-main">
  <!-- sidebar start -->
  <jsp:include page="/WEB-INF/views/public/sidebar.jsp"></jsp:include>
  <!-- sidebar end -->

<!-- content start -->
<div class="admin-content">
  <div class="admin-content-body">
    <div class="am-cf am-padding am-padding-bottom-0">
      <div class="am-fl am-cf">
        <strong class="am-text-primary am-text-lg">考勤统计</strong> /
        <small>checkout</small>
      </div>
    </div>

    <hr>
    <%
    if(request.getAttribute("status").equals("new")) {
    	%>
    	&nbsp;<button class="am-btn am-btn-primary" onclick="location.href='<%=request.getContextPath() %>/webadmin/checkwork.htm?user_id=<%=request.getParameter("user_id")%>&time=old'">查看上月考勤统计</button>
    <div class="am-tabs am-margin" data-am-tabs>
    
              本月考勤统计：
      <%
      List<CheckWork> cwList = (List<CheckWork>) request.getAttribute("cwList");
      DataTypeConver dtc = new DataTypeConver();
      if(cwList == null || cwList.size() == 0) {
    	  out.print("没有查询到考勤数据！");
      } else {
    	  %>
    	  <br /><br />日期&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;星期&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;上午打卡时间&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;下午打卡时间
    	  <%
    	  int num = 0;
    	  for(CheckWork cw:cwList) {
    		  %>
    		  <br />
    		  <%=cw.getSign_date() %>  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<%=cw.getSign_week() %> &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    		  <% 
    		  if(cw.getMorn_time() == 0) { out.print("<span style='color:red;'>上午未打卡！</span>"); } else { out.print(dtc.timeToString(cw.getMorn_time(), "yyyy-MM-dd HH:mm:ss")); num++;}
    		  out.print("&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;");
    		  if(cw.getAft_time() == 0) { out.print("<span style='color:red;'>下午未打卡！</span>"); } else { out.print(dtc.timeToString(cw.getAft_time(), "yyyy-MM-dd HH:mm:ss")); num++;}
    		  %>
    		  <%
    	  }
    	  DecimalFormat df = new DecimalFormat("0.00");
    	  out.print("<br /><br />本月考勤数据统计：<br />&nbsp;&nbsp;&nbsp;" +
    	   "&nbsp;&nbsp;&nbsp;工作天数：" + (num/2) + "天&nbsp&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;出勤率：" + df.format(((int)(num/2)/(cwList.size()*1.0))*100) + "%");
    	  out.print("<br />&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;应发工资：" + (df.format((int)(num/2) * 303.23)));
      }
      %>
    </div>
    	<%
    } else {
    	%>
    	&nbsp;<button class="am-btn am-btn-primary" onclick="location.href='<%=request.getContextPath() %>/webadmin/checkwork.htm?user_id=<%=request.getParameter("user_id")%>'">查看本月考勤统计</button>
    <div class="am-tabs am-margin" data-am-tabs>
    
             上月考勤统计：
      <%
      List<CheckWork> cwList = (List<CheckWork>) request.getAttribute("cwList");
      DataTypeConver dtc = new DataTypeConver();
      if(cwList == null || cwList.size() == 0) {
    	  out.print("没有查询到考勤数据！");
      } else {
    	  %>
    	  <br /><br />日期&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;星期&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;上午打卡时间&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;下午打卡时间
    	  <%
    	  int num = 0;
    	  for(CheckWork cw:cwList) {
    		  %>
    		  <br />
    		  <%=cw.getSign_date() %>  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<%=cw.getSign_week() %> &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    		  <% 
    		  if(cw.getMorn_time() == 0) { out.print("<span style='color:red;'>上午未打卡！</span>"); } else { out.print(dtc.timeToString(cw.getMorn_time(), "yyyy-MM-dd HH:mm:ss")); num++;}
    		  out.print("&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;");
    		  if(cw.getAft_time() == 0) { out.print("<span style='color:red;'>下午未打卡！</span>"); } else { out.print(dtc.timeToString(cw.getAft_time(), "yyyy-MM-dd HH:mm:ss")); num++;}
    		  %>
    		  <%
    	  }
    	  DecimalFormat df = new DecimalFormat("0.00");
    	  out.print("<br /><br />上月考勤数据统计：<br />&nbsp;&nbsp;&nbsp;" +
    	   "&nbsp;&nbsp;&nbsp;工作天数：" + (num/2) + "天&nbsp&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;出勤率：" + df.format(((int)(num/2)/(cwList.size()*1.0))*100) + "%");
    	  out.print("<br />&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;应发工资：" + (df.format((int)(num/2) * 303.23)));
      }
      %>
    </div>
    	<%
    }
    %>
  </div>

  <!-- 底部版权信息 -->
  <jsp:include page="/WEB-INF/views/public/footer.jsp"></jsp:include>
</div>
<!-- content end -->

</div>

<a href="#" class="am-icon-btn am-icon-th-list am-show-sm-only admin-menu" data-am-offcanvas="{target: '#admin-offcanvas'}"></a>

<jsp:include page="/WEB-INF/views/public/scripts.jsp">
	<jsp:param value="<%=pagePath %>" name="pagePath"/>
</jsp:include>
<!-- 引入layer 组件 -->
<script type="text/javascript" src="<%=pagePath %>/static/layer/layer.js"></script>
<script type="text/javascript">
//查看密码功能
$("#cat_password").mousedown(function() {
	$("#new_password").attr("type","text");
	$("#repeat_password").attr("type","text");
});
$("#cat_password").mouseup(function() {
	$("#new_password").attr("type","password");
	$("#repeat_password").attr("type","password");
});
//修改密码
$("#editpasswd_commit").click(function() {
	var old_password = $("#old_password").val();
	var new_password = $("#new_password").val();
	var repeat_password = $("#repeat_password").val();
	if(old_password == "" || new_password == "" || repeat_password == "") {
		layer.msg("密码不能为空！");
		//光标回到输入焦点
		$("#old_password").focus();
		return false;
	}
	//加载提示 start
	var index = layer.load(0, { shade: [0.3,'#fff'] //0.1透明度的白色背景
		});
	$.ajax({
        type:"post",
        async:true,
        url:"<%=pagePath %>/webadmin/configs/editpasswd",
        //dataType:"html",
        data:{"old_password":old_password,"new_password":new_password,"repeat_password":repeat_password},
        success:function(data) {
        	//请求成功 关闭loding
            layer.closeAll('loading');
        	if(data == "empty") {
        		layer.msg("密码不能为空！");
        		return false;
        	}
        	if(data == "no_agr") {
        		layer.msg("两次密码不一致！请检查...");
        		return false;
        	}
        	if(data == "password_dv") {
        		layer.msg("新密码不符合要求，请检查!");
        		return false;
        	}
        	if(data == "passwd_error") {
        		layer.msg("原密码错误，请重新输入！");
        		return false;
        	}
        	if(data == "success") {
        		//alert("密码修改成功，请牢记！");
        		layer.alert('密码修改成功，请牢记！', {
					  skin: 'layui-layer-lan' //样式类名
					  ,closeBtn: 0
					}, function(){
					  location.reload();
					});
        		return true;
        	}
        	if(data == "error") {
        		layer.msg("密码修改失败，请稍后再试...");
        		return false;
        	}
        },
        error:function() {
        	//请求成功 关闭loding
            layer.closeAll('loading');
        	layer.msg("抱歉，请求失败！请稍后再试..");
        	return false;
        }
	});
});
</script>
</body>
</html>
