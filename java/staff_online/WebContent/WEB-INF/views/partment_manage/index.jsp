<%@page import="net.zengzhiying.tools.DataTypeConver"%>
<%@page import="net.zengzhiying.beans.PartToNumber"%>
<%@ page language="java" contentType="text/html; charset=UTF-8"
    pageEncoding="UTF-8" import="java.util.List,net.zengzhiying.beans.UserToPart" %>
<%
	String pagePath = request.getContextPath(); //获取页面通用的path上下文
%>
<!doctype html>
<html class="no-js">
<head>
  <meta charset="utf-8">
  <meta http-equiv="X-UA-Compatible" content="IE=edge">
  <title>部门经理列表 - 员工上线后台管理系统</title>
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
        <div class="am-fl am-cf"><strong class="am-text-primary am-text-lg">表格</strong> / <small>Table</small></div>
      </div>

      <hr>

      <div class="am-g">
        <div class="am-u-sm-12 am-u-md-6">
          <div class="am-btn-toolbar">
            <div class="am-btn-group am-btn-group-xs">
              <button type="button" class="am-btn am-btn-default" onclick="location.href='<%=pagePath %>/webadmin/partment_manage/add'"><span class="am-icon-plus"></span> 新增经理</button>
            </div>
          </div>
        </div>
      </div>

      <div class="am-g">
        <div class="am-u-sm-12">
          <form class="am-form">
            <table class="am-table am-table-striped am-table-hover table-main">
              <thead>
              <tr>
                <th class="">经理编号</th><th class="table-title">姓名</th><th class="table-type">联系方式</th><th class="table-author am-hide-sm-only">负责部门</th><th class="table-date am-hide-sm-only">部门员工数</th><th class="table-set">操作</th>
              </tr>
              </thead>
              <tbody>
              <%
              	List<PartToNumber> pnList = (List<PartToNumber>) request.getAttribute("staffList");
              	if(pnList == null ||pnList.size() == 0) {
              		%>
              		<p style="font-size:22px;">暂时没有部门经理，添加一个吧！</p>
              		<%
              	} else {
              		for(PartToNumber pn:pnList) {
              		%>
              		<tr>
		                <td><%=pn.getUtp().getUser().getUser_id() %></td>
		                <td><a href="javascript:;" onclick="show_info('staff_info_<%=pn.getUtp().getUser().getUser_id() %>')"><%=pn.getUtp().getUser().getUser_name() %></a>
		                <span id="staff_info_<%=pn.getUtp().getUser().getUser_id() %>" style="display:none;">
		                	<%
		                	DataTypeConver dtc = new DataTypeConver();
		                	
		                	%>
		                	
		                	&nbsp;&nbsp;入职时间：<%=dtc.timeToString(pn.getUtp().getUser().getCommit_time(), "yyyy-MM-dd HH:mm:ss") %>
		                	<br />
		                	&nbsp;&nbsp;工龄：<%
		          				int a = (int) ((dtc.newTime() - pn.getUtp().getUser().getCommit_time())/(3600 * 24.0));
		                	out.print(a + "天");
		                	%>
		                	<br />
		                	&nbsp;&nbsp;员工介绍：
		                	<%
		                	if(pn.getUtp().getUser().getUser_describe().equals("0")) {
		                		out.print("暂无");
		                	} else {
		                		out.print(pn.getUtp().getUser().getUser_describe());
		                	}
		                	%>
		                	<br /><br />
		                	&nbsp;
		                </span>
		                </td>
		                <td><%
		                if(pn.getUtp().getUser().getContact().equals("0")) {
		                	out.print("暂无");
		                } else {
		                	out.print(pn.getUtp().getUser().getContact());
		                }
		                %></td>
		                <td class="am-hide-sm-only"><%=pn.getUtp().getPm().getPm_name() %></td>
		                <td class="am-hide-sm-only"><%=pn.getNumber() %></td>
		                <td>
		                  <div class="am-btn-toolbar">
		                    <div class="am-btn-group am-btn-group-xs">
		                    	<%
		                    	if(pn.getUtp().getUser().getIs_approve()  == 0) {
		                    		%>
		                    		<button class="am-btn am-btn-default am-btn-xs am-hide-sm-only"><span class="am-icon-copy"></span> 通过审批</button>
		                    		<%
		                    	}
		                    	%>
		                      <a class="am-btn am-btn-default am-btn-xs am-text-secondary" onclick="location.href='<%=pagePath %>/webadmin/partment_manage/edit?user_id=<%=pn.getUtp().getUser().getUser_id() %>'"><span class="am-icon-pencil-square-o"></span> 编辑详情</a>
		                      <a class="am-btn am-btn-default am-btn-xs am-text-danger am-hide-sm-only" href="javascript:;" onclick="deleteStaff(<%=pn.getUtp().getUser().getUser_id() %>)"><span class="am-icon-trash-o"></span> 删除</a>
		                    </div>
		                  </div>
		                </td>
		              </tr>
              		<%
              		}
              	}
              %>
              </tbody>
            </table>
            <div class="am-cf">
              共<%=pnList.size() %> 条记录
              <div class="am-fr">
              <!-- 
                <ul class="am-pagination">
                  <li class="am-disabled"><a href="#">«</a></li>
                  <li class="am-active"><a href="#">1</a></li>
                  <li><a href="#">2</a></li>
                  <li><a href="#">3</a></li>
                  <li><a href="#">4</a></li>
                  <li><a href="#">5</a></li>
                  <li><a href="#">»</a></li>
                </ul>
                 -->
              </div>
            </div>
            <hr />
            <p>注：.....</p>
          </form>
        </div>

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
// 删除用户脚本处理
function deleteStaff(user_id) {
	
	layer.confirm('您确认删除吗？删除之后无法恢复!', {
		  btn: ['确定','取消'] //按钮
		}, function(){
			//加载提示 start
			var index = layer.load(0, { shade: [0.3,'#fff'] //0.1透明度的白色背景
				});
			//删除请求
			//执行删除
			$.ajax({
	            type:"post",
	            async:true,
	            url:"<%=pagePath %>/webadmin/partment_manage/edit",
	            //dataType:"html",
	            data:{"user_id":user_id,"action":"delete"},
	            success:function(data) {
	            	//请求成功 关闭loding
		            layer.closeAll('loading');
	              if(data == "param_error") {
	            	  layer.msg("参数错误！");
	            	  return false;
	              }
	              if(data == "super_error") {
	            	  layer.msg("严重错误！不能删除超级管理员..");
	            	  return false;
	              }
	              if(data == "success") {
	            	  layer.msg("删除成功！");
	            	  setTimeout("location.reload()", 2000);
	            	  return true;
	              }
	              if(data == "error") {
	            	  layer.msg("删除失败！请稍后再试....");
	            	  return false;
	              }
	              return;
	            },
	            error:function() {
	            	//请求成功 关闭loding
		            layer.closeAll('loading');
	            	alert("抱歉，请求失败！请稍后再试....");
	            	return false;
	            }
	          });
		}, function(){
			layer.msg("您未选择删除！");
			return false;
		});
	
	return;
	var r = confirm("您确认删除吗？删除之后无法恢复!");
	if(r == true) {
		//删除请求
		//执行删除
		$.ajax({
            type:"post",
            async:true,
            url:"<%=pagePath %>/webadmin/partment_manage/edit",
            //dataType:"html",
            data:{"user_id":user_id,"action":"delete"},
            success:function(data) {
              if(data == "param_error") {
            	  alert("参数错误！");
            	  return false;
              }
              if(data == "super_error") {
            	  alert("严重错误！不能删除超级管理员..");
            	  return false;
              }
              if(data == "success") {
            	  alert("删除成功！");
            	  return true;
              }
              if(data == "error") {
            	  alert("删除失败！请稍后再试....");
            	  return false;
              }
              return;
            },
            error:function() {
            	alert("抱歉，请求失败！请稍后再试....");
            	return false;
            }
          });
	} else {
		//
		alert("您未选择删除！");
		return false;
	}
}
</script>

<script type="text/javascript">
  //员工信息输出
  function show_info(info_id) {
	//自定页
	  layer.open({
	    type: 1,
	    title:'部门经理信息',
	    skin: 'layui-layer-molv', //样式类名
	    closeBtn: 0, //不显示关闭按钮
	    shift: 2,
	    shadeClose: true, //开启遮罩关闭
	    content: $("#" + info_id).html()
	  });
  }
  </script>
</body>
</html>
