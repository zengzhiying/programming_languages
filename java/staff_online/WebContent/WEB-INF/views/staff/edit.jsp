<%@page import="net.zengzhiying.beans.User"%>
<%@page import="net.zengzhiying.beans.Partment"%>
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
        <div class="am-fl am-cf"><strong class="am-text-primary am-text-lg">员工编辑界面</strong> / <small>edit</small></div>
      </div>
      <hr>
      
      <form class="am-form">
  <fieldset>
  <%
   User u = (User) request.getAttribute("user");
  	if(u != null) {
  		%>
  		<div class="am-form-group">
      <label for="doc-ipt-email-1">员工姓名</label>
      <input type="text" class="" value="<%=u.getUser_name() %>" name="user_name" id="user_name" placeholder="输入员工真实姓名">
    </div>

    <div >
    	<label for="doc-ipt-email-1">请确认性别：</label>
      <label class="am-radio">
      <input type="radio" name="gender" value="1" data-am-ucheck <% if(u.getGender() == 1) { %>checked<% } %>>
      男
    </label>
    <label class="am-radio">
      <input type="radio" name="gender" value="0" data-am-ucheck <% if(u.getGender() == 0) { %>checked<% } %>>
      女
    </label>
    </div>
    
    <div class="am-form-group">
      <label for="doc-select-1">请填写员工年龄</label>
      <input type="text" class="" name="age" id="age" value="<%=u.getAge() %>" placeholder="请输入员工年龄">
    </div>

    <div class="am-form-group">
      <%
      if(request.getSession().getAttribute("super_type") != null) {
    	  %>
    	  <label for="doc-select-1">请选择部门</label>
    	  <%
      List<Object> pmList = (List<Object>) request.getAttribute("pmList");
      if(pmList != null && pmList.size() > 0) {
      %>
      <select id="partment_id">
      <% for(Object obj:pmList) { 
    	  Partment pm = (Partment) obj;
      %>
        <option value="<%=pm.getPm_id() %>"><%=pm.getPm_name() %></option>
        <% } %>
      </select>
      <% } 
  	}
      %>
      <span class="am-form-caret"></span>
    </div>
    
    <div class="am-form-group">
      <label for="doc-select-1">请填写联系方式</label>
      <input type="text" class="" value="<%=u.getContact() %>" name="contact" id="contact" placeholder="请填写联系方式">
    </div>

    <div class="am-form-group">
      <label for="doc-ta-1">请填写员工简介</label>
      <textarea name="user_describe" id="user_describe" class="" rows="5" id="doc-ta-1" placeholder="介绍一下员工吧"><%
      if(!u.getUser_describe().equals("0")) {
    	  out.print(u.getUser_describe());
      }
      %></textarea>
    </div>
    <input type="hidden" id="user_id" value="<%=u.getUser_id() %>" >
    <input type="hidden" id="action" value="edit">
    <p><button type="button" class="am-btn am-btn-secondary" id="user_commit" style="width:136px;">提交</button></p>
  		<%
  	} else {
  %>
    <div class="am-form-group">
      <label for="doc-ipt-email-1">员工姓名</label>
      <input type="text" class="" name="user_name" id="user_name" placeholder="输入员工真实姓名">
    </div>

    <div >
    	<label for="doc-ipt-email-1">请确认性别：</label>
      <label class="am-radio">
      <input type="radio" name="gender" value="1" data-am-ucheck checked>
      男
    </label>
    <label class="am-radio">
      <input type="radio" name="gender" value="0" data-am-ucheck>
      女
    </label>
    </div>
    
    <div class="am-form-group">
      <label for="doc-select-1">请填写员工年龄</label>
      <input type="text" class="" name="age" id="age" placeholder="请输入员工年龄">
    </div>

    <div class="am-form-group">
      
      <%
      if(request.getSession().getAttribute("super_type") != null) {
    	  %>
    	  <label for="doc-select-1">请选择部门</label>
    	  <%
      List<Object> pmList = (List<Object>) request.getAttribute("pmList");
      if(pmList != null && pmList.size() > 0) {
      %>
      <select id="partment_id">
      <% for(Object obj:pmList) { 
    	  Partment pm = (Partment) obj;
      %>
        <option value="<%=pm.getPm_id() %>"><%=pm.getPm_name() %></option>
        <% } %>
      </select>
      <% } 
      }
      %>
      <span class="am-form-caret"></span>
    </div>
    
    <div class="am-form-group">
      <label for="doc-select-1">请填写联系方式</label>
      <input type="text" class="" name="contact" id="contact" placeholder="请填写联系方式">
    </div>

    <div class="am-form-group">
      <label for="doc-ta-1">请填写员工简介</label>
      <textarea name="user_describe" id="user_describe" class="" rows="5" id="doc-ta-1" placeholder="介绍一下员工吧"></textarea>
    </div>
    <input type="hidden" id="user_id" value="" >
    <p><button type="button" class="am-btn am-btn-secondary" id="user_commit" style="width:136px;">提交</button></p>
    <% } %>
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
	<%
	if(u != null) {
		//选中原来的下拉列表
		%>
		$("#partment_id").val(<%=u.getDepartment_id() %>);
		<%
	}
	%>
	
	$("#user_commit").click(function() {
		var user_name = $("#user_name").val();
		//var gender = $("#gender").val();
		var genderObj = document.getElementsByName("gender");
		var gender;
		for(var i = 0;i < genderObj.length;i++) {
			if(genderObj[i].checked) {
				gender = genderObj[i].value;
			}
		}
		var partment_id = $("#partment_id").val();
		var contact = $("#contact").val();
		var user_describe = $("#user_describe").val();
		var age = $("#age").val();
		
		//修改字段
		var user_id = $("#user_id").val();
		var action = $("#action").val();
		/*
		if(pm_name == "") {
			alert("请输入部门名称！");
			return false;
		}
		if(setup_time == "") {
			alert("请选择成立时间！");
			return false;
		}
		*/
		if(typeof user_id === 'undefined' || user_id == "") {
			//加载提示 start
			var index = layer.load(0, { shade: [0.3,'#fff'] //0.1透明度的白色背景
				});
			$.ajax({
	            type:"post",
	            async:true,
	            url:"<%=pagePath %>/webadmin/staffadd",
	            //dataType:"html",
	            data:{"user_name":user_name,"gender":gender,"age":age,"partment_id":partment_id,"contact":contact,"user_describe":user_describe},
	            success:function(data) {
	            	//请求成功 关闭loding
		            layer.closeAll('loading');
	              if(data == "empty") {
	            	  layer.msg("必填内容不能为空！");
	            	  return false;
	              }
	              if(data == "param_error") {
	            	  layer.msg("参数选择错误！");
	            	  return false;
	              }
	              if(data == "age_error") {
	            	  layer.msg("年龄超出范围！");
	            	  return false;
	              }
	              if(data == "success") {
	            	  //alert("新增员工成功！");
	            	  layer.alert('新增员工成功！', {
						  skin: 'layui-layer-lan' //样式类名
						  ,closeBtn: 0
						}, function(){
						  location.href="<%=request.getContextPath() %>/webadmin/stafflist";
						});
	            	  return true;
	              }
	              if(data == "error") {
	            	  layer.msg("添加员工失败！请检查输入或稍后再试....");
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
		} else {
			var index = layer.load(0, { shade: [0.3,'#fff'] //0.1透明度的白色背景
			});
			$.ajax({
	            type:"post",
	            async:true,
	            url:"<%=pagePath %>/webadmin/staffedit",
	            //dataType:"html",
	            data:{"user_name":user_name,"gender":gender,"age":age,"partment_id":partment_id,"contact":contact,"user_describe":user_describe,"user_id":user_id,"action":action},
	            success:function(data) {
	            	//请求成功 关闭loding
		            layer.closeAll('loading');
	              if(data == "empty") {
	            	  layer.msg("必填内容不能为空！");
	            	  return false;
	              }
	              if(data == "param_error") {
	            	  layer.msg("参数选择错误！");
	            	  return false;
	              }
	              if(data == "age_error") {
	            	  layer.msg("年龄超出范围！");
	            	  return false;
	              }
	              if(data == "success") {
	            	  //alert("修改员工成功！");
	            	  layer.alert('修改员工成功！', {
						  skin: 'layui-layer-lan' //样式类名
						  ,closeBtn: 0
						}, function(){
						  location.href="<%=request.getContextPath() %>/webadmin/stafflist";
						});
	            	  return true;
	              }
	              if(data == "error") {
	            	//请求成功 关闭loding
			            layer.closeAll('loading');
			            layer.msg("修改员工失败！请检查输入或稍后再试....");
	            	  return false;
	              }
	              return;
	            },
	            error:function() {
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
