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
  <title>员工上线后台管理系统</title>
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
        <strong class="am-text-primary am-text-lg">修改密码</strong> /
        <small>form</small>
      </div>
    </div>

    <hr>

    <div class="am-tabs am-margin" data-am-tabs>
      <ul class="am-tabs-nav am-nav am-nav-tabs">
        <li><a href="#tab2">密码修改</a></li>
      </ul>

      <div class="am-tabs-bd">
        

        <div class="am-tab-panel am-fade" id="tab2">
          <form class="am-form">
            <div class="am-g am-margin-top">
              <div class="am-u-sm-4 am-u-md-2 am-text-right">
              旧密码
              </div>
              <div class="am-u-sm-8 am-u-md-4">
                <input type="password" class="am-input-sm" id="old_password">
              </div>
              <div class="am-hide-sm-only am-u-md-6">*必填</div>
            </div>

            <div class="am-g am-margin-top">
              <div class="am-u-sm-4 am-u-md-2 am-text-right">
               新密码
              </div>
              <div class="am-u-sm-8 am-u-md-4 am-u-end col-end">
                <input type="password" class="am-input-sm" id="new_password">
              </div>
              <div class="am-hide-sm-only am-u-md-6">*必填 </div>
            </div>
            
            <div class="am-g am-margin-top">
              <div class="am-u-sm-4 am-u-md-2 am-text-right">
              重复 新密码
              </div>
              <div class="am-u-sm-8 am-u-md-4 am-u-end col-end">
                <input type="password" class="am-input-sm" id="repeat_password">
              </div>
              <div class="am-hide-sm-only am-u-md-6"><a href="javascript:;" id="cat_password">查看</a> &nbsp*必填</div>
            </div>

            

          </form>
        </div>

        

      </div>
    </div>

    <div class="am-margin">
      <button type="button" class="am-btn am-btn-primary am-btn-xs" style="width:160px; height:32px;" id="editpasswd_commit">提交保存</button>
      <button type="button" class="am-btn am-btn-primary am-btn-xs" style="width:160px; height:32px;" onclick="history.back()">放弃修改</button>
    </div>
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
