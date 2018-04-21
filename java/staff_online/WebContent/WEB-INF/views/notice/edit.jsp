<%@page import="net.zengzhiying.beans.Notice"%>
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
  <title>公告编辑- 员工上线后台管理系统</title>
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
  
   <link href="<%=pagePath %>/umeditor/themes/default/css/umeditor.css" type="text/css" rel="stylesheet">
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
        <div class="am-fl am-cf"><strong class="am-text-primary am-text-lg">公告编辑界面</strong> / <small>edit</small></div>
      </div>
      <hr>
      
      <form class="am-form">
  <fieldset>

    <div class="am-form-group">
      <label for="doc-ipt-email-1">公告标题</label>
      <% if(request.getAttribute("nt") == null) { %>
      <input type="text" class="" name="nt_title" id="nt_title" placeholder="请输入公告标题">
      <% } else {
    	  Notice nt =  (Notice) request.getAttribute("nt");
    	  %>
      <input type="text" class="" value="<%=nt.getNt_title() %>" name="nt_title" id="nt_title" placeholder="请输入公告标题">
      <% } %>
    </div>


    <div class="am-form-group">
      <label for="doc-ta-1">公告摘要</label>
      <% if(request.getAttribute("nt") == null) { %>
      <textarea name="nt_desc" class="" rows="5" id="nt_desc" placeholder="填写简短的摘要"></textarea>
      <% } else {
    	  Notice nt =  (Notice) request.getAttribute("nt");
    	  %>
    	  <textarea name="nt_desc" class="" rows="5" id="nt_desc" placeholder="填写简短的摘要"><%=nt.getNt_desc() %></textarea>
    	  <%  } %>
    </div>
    <div class="am-form-group">
      <label for="doc-ta-1">公告内容</label>
      <% if(request.getAttribute("nt") == null) { %>
      <textarea name="nt_content" class="" rows="8" id="nt_content" placeholder="请编写公告正文"></textarea>
      <% } else {
    	  Notice nt =  (Notice) request.getAttribute("nt");
    	  %>
    	  <textarea name="nt_content" class="" rows="8" id="nt_content" placeholder="请编写公告正文"><%=nt.getNt_content() %></textarea>
    	  <input type="hidden" name="nt_id" id="nt_id" value="<%=nt.getNt_id() %>">
    	  <%  } %>
    </div>
    <p><button type="button" class="am-btn am-btn-primary" style="width:168px;" id="partment_commit">发布公告</button></p>
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
<!-- 引入um -->
<!-- 配置文件 -->
    <script type="text/javascript" src="<%=pagePath %>/umeditor/umeditor.config.js"></script>
    <!-- 编辑器源码文件 -->
    <script type="text/javascript" src="<%=pagePath %>/umeditor/umeditor.min.js"></script>
    <!-- 实例化编辑器 -->
    <script type="text/javascript">
        var um = UM.getEditor('nt_content');
    </script>
<!-- 引入layer 组件 -->
<script type="text/javascript" src="<%=pagePath %>/static/layer/layer.js"></script>
<script type="text/javascript">
//js 提交控制
$(document).ready(function(){
	$(partment_commit).click(function() {
		var nt_title = $("#nt_title").val();
		var nt_desc = $("#nt_desc").val();
		var nt_content = $("#nt_content").val();
		var nt_id = $("#nt_id").val();
		if(nt_title == "" || nt_desc == "" || nt_content == "") {
			//alert("请输入部门名称！");
			layer.msg('必填内容不能为空！');
			return false;
		}
		if(typeof nt_id === 'undefined' || nt_id == "") {
			//加载提示 start
			var index = layer.load(0, { shade: [0.3,'#fff'] //0.1透明度的白色背景
				});
			$.ajax({
	            type:"post",
	            async:true,
	            url:"<%=pagePath %>/webadmin/notice/add",
	            //dataType:"html",
	            data:{"nt_title":nt_title,"nt_desc":nt_desc,"nt_content":nt_content},
	            success:function(data) {
	            //请求成功 关闭loding
	            layer.closeAll('loading');
	              if(data == "success") {
	            	  //alert("编辑部门成功！");
	            	  layer.alert('发布公告成功！', {
						  skin: 'layui-layer-lan' //样式类名
						  ,closeBtn: 0
						}, function(){
						  location.href="<%=request.getContextPath() %>/webadmin/notice.htm";
						});
	            	  return true;
	              }
	              if(data == "error") {
	            	  //alert("编辑部门失败！请检查输入或稍后再试....");
	            	  layer.msg('发布公告失败，请检查输入或稍后再试....');
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
	            url:"<%=pagePath %>/webadmin/notice/edit",
	            //dataType:"html",
	            data:{"nt_title":nt_title,"nt_desc":nt_desc,"nt_content":nt_content,"nt_id":nt_id},
	            success:function(data) {
	            	//请求成功 关闭loding
		            layer.closeAll('loading');
	              
	              if(data == "success") {
	            	  layer.alert('编辑公告成功！', {
						  skin: 'layui-layer-lan' //样式类名
						  ,closeBtn: 0
						}, function(){
						  location.href="<%=request.getContextPath() %>/webadmin/notice.htm";
						});
	            	  return true;
	              }
	              if(data == "error") {
	            	  layer.msg("编辑失败！请检查....");
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
