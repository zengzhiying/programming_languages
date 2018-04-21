<%@page import="net.zengzhiying.tools.DataTypeConver"%>
<%@page import="net.zengzhiying.beans.Partment"%>
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
  <title>员工列表 - 员工上线后台管理系统</title>
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
    <div class="admin-content-body" style="margin-left:36px;">
      <div class="am-cf am-padding am-padding-bottom-0">
        <div class="am-fl am-cf"><strong class="am-text-primary am-text-lg">部门编辑界面</strong> / <small>edit</small></div>
      </div>
      <hr>
      
      <form class="am-form">
  <fieldset>

    <div class="am-form-group">
      <label for="doc-ipt-email-1">部门名称</label>
      <% if(request.getAttribute("pm") == null) { %>
      <input type="text" class="" name="pm_name" id="pm_name" placeholder="请输入部门名称">
      <% } else {
    	  Partment pm =  (Partment) request.getAttribute("pm");
    	  %>
      <input type="text" class="" value="<%=pm.getPm_name() %>" name="pm_name" id="pm_name" placeholder="请输入部门名称">
      <% } %>
    </div>


    <div class="am-form-group">
      <label for="doc-ta-1">部门简介</label>
      <% if(request.getAttribute("pm") == null) { %>
      <textarea name="pm_describe" class="" rows="5" id="pm_describe" placeholder="介绍一下部门吧"></textarea>
      <% } else {
    	  Partment pm =  (Partment) request.getAttribute("pm");
    	  if(pm.getPm_describe().equals("0")) {
    	  %>
    	  <textarea name="pm_describe" class="" rows="5" id="pm_describe" placeholder="介绍一下部门吧"></textarea>
    	  <% } else { %>
    	  <textarea name="pm_describe" class="" rows="5" id="pm_describe" placeholder="介绍一下部门吧"><%=pm.getPm_describe() %></textarea>
    	  <% } } %>
    </div>
    <label for="doc-ta-1">请选择部门成立时间</label>
    <% if(request.getAttribute("pm") == null) { %>
    <input type="text" name="setup_time" id="setup_time" class="am-form-field" placeholder="选择部门成立时间" data-am-datepicker readonly required />
    <input type="hidden" name="pm_id" id="pm_id" value="">
    <% } else {
    	  Partment pm =  (Partment) request.getAttribute("pm");
    	  %>
    <input type="text" name="setup_time" id="setup_time" class="am-form-field" value="<%=(new DataTypeConver().timeToString(pm.getSetup_time(), "yyyy-MM-dd")) %>" placeholder="选择部门成立时间" data-am-datepicker readonly required />
    <input type="hidden" name="pm_id" id="pm_id" value="<%=pm.getPm_id() %>">
    <input type="hidden" name="action" id="action" value="edit" />
    <% } %>
    <p><button type="button" class="am-btn am-btn-primary" style="width:168px;" id="partment_commit">提交</button></p>
  </fieldset>
</form>

      

      </div>
    </div>

  <!-- 底部版权信息 -->
  <jsp:include page="/WEB-INF/views/public/footer.jsp"></jsp:include>
</div>
<!-- content end -->


<a href="#" class="am-icon-btn am-icon-th-list am-show-sm-only admin-menu" data-am-offcanvas="{target: '#admin-offcanvas'}"></a>

<jsp:include page="/WEB-INF/views/public/scripts.jsp">
	<jsp:param value="<%=pagePath %>" name="pagePath"/>
</jsp:include>
<!-- 引入layer 组件 -->
<script type="text/javascript" src="<%=pagePath %>/static/layer/layer.js"></script>
<script type="text/javascript">
//js 提交控制
$(document).ready(function(){
	$(partment_commit).click(function() {
		var pm_name = $("#pm_name").val();
		var pm_describe = $("#pm_describe").val();
		var setup_time = $("#setup_time").val();
		var pm_id = $("#pm_id").val();
		var action = $("#action").val();
		if(pm_name == "") {
			//alert("请输入部门名称！");
			layer.msg('请输入部门名称！');
			return false;
		}
		if(setup_time == "") {
			//alert("请选择成立时间！");
			layer.msg('请选择成立时间！');
			return false;
		}
		if(typeof pm_id === 'undefined' || pm_id == "") {
			//加载提示 start
			var index = layer.load(0, { shade: [0.3,'#fff'] //0.1透明度的白色背景
				});
			$.ajax({
	            type:"post",
	            async:true,
	            url:"<%=pagePath %>/webadmin/partment/add",
	            //dataType:"html",
	            data:{"pm_name":pm_name,"setup_time":setup_time,"pm_describe":pm_describe},
	            success:function(data) {
	            //请求成功 关闭loding
	            layer.closeAll('loading');
	              if(data == "empty") {
	            	  //alert("必填内容不能为空！");
	            	  layer.msg('必填内容不能为空！');
	            	  return false;
	              }
	              if(data == "time_error") {
	            	  //alert("时间格式错误！请重新选择.");
	            	  layer.msg('时间格式错误！请重新选择.', function(){
	            		  //关闭后的操作
	            		  });
	            	  return false;
	              }
	              if(data == "success") {
	            	  //alert("编辑部门成功！");
	            	  layer.alert('添加部门成功！', {
						  skin: 'layui-layer-lan' //样式类名
						  ,closeBtn: 0
						}, function(){
						  location.href="<%=request.getContextPath() %>/webadmin/partment";
						});
	            	  return true;
	              }
	              if(data == "error") {
	            	  //alert("编辑部门失败！请检查输入或稍后再试....");
	            	  layer.msg('编辑部门失败！请检查输入或稍后再试....');
	            	  return false;
	              }
	              return;
	            },
	            error:function() {
	            	//请求成功 关闭loding
		            layer.closeAll('loading');
	            	//alert("抱歉，请求失败！请稍后再试....");
	            	layer.msg('抱歉，请求失败！请稍后再试....');
	            	return;
	            }
	          });
		} else {
			$.ajax({
	            type:"post",
	            async:true,
	            url:"<%=pagePath %>/webadmin/partment/edit",
	            //dataType:"html",
	            data:{"pm_name":pm_name,"setup_time":setup_time,"pm_describe":pm_describe,"pm_id":pm_id,"action":action},
	            success:function(data) {
	            	//请求成功 关闭loding
		            layer.closeAll('loading');
	              if(data == "empty") {
	            	  layer.msg("必填内容不能为空！");
	            	  return false;
	              }
	              if(data == "time_error") {
	            	  layer.msg("时间格式错误！请重新选择.");
	            	  return false;
	              }
	              if(data == "id_error") {
	            	  layer.msg("参数错误！请重试");
	            	  return false;
	              }
	              if(data == "success") {
	            	  layer.alert('编辑部门成功！', {
						  skin: 'layui-layer-lan' //样式类名
						  ,closeBtn: 0
						}, function(){
						  location.href="<%=request.getContextPath() %>/webadmin/partment";
						});
	            	  return true;
	              }
	              if(data == "error") {
	            	  layer.msg("修改失败！可能是内容并未改变....");
	            	  return false;
	              }
	              return;
	            },
	            error:function() {
	            	//请求成功 关闭loding
		            layer.closeAll('loading');
	            	layer.msg("抱歉，请求失败！请稍后再试....");
	            	return;
	            }
	          });
		}
	});
});
</script>

</body>
</html>
