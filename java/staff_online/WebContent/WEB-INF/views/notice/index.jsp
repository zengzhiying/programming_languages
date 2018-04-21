<%@page import="net.zengzhiying.beans.Notice"%>
<%@page import="net.zengzhiying.tools.DataTypeConver"%>
<%@page import="java.text.SimpleDateFormat"%>
<%@ page language="java" contentType="text/html; charset=UTF-8"
    pageEncoding="UTF-8" import="java.util.List,net.zengzhiying.beans.Partment" %>
<%
	String pagePath = request.getContextPath(); //获取页面通用的path上下文
%>
<!doctype html>
<html class="no-js">
<head>
  <meta charset="utf-8">
  <meta http-equiv="X-UA-Compatible" content="IE=edge">
  <title>公告列表 - 员工上线后台管理系统</title>
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
        <div class="am-fl am-cf"><strong class="am-text-primary am-text-lg">公告管理</strong> / <small>Table</small></div>
      </div>

      <hr>

      <div class="am-g">
        <div class="am-u-sm-12 am-u-md-6">
          <div class="am-btn-toolbar">
          <% if(request.getSession().getAttribute("super_type") != null) { %>
            <div class="am-btn-group am-btn-group-xs">
              <button type="button" class="am-btn am-btn-default" onclick="location.href='<%=pagePath %>/webadmin/notice/add'"><span class="am-icon-plus"></span> 发布新公告</button>
            </div>
            <% } %>
          </div>
        </div>
      </div>

      <div class="am-g">
        <div class="am-u-sm-12">
          <form class="am-form">
            <table class="am-table am-table-striped am-table-hover table-main">
              <thead>
              <tr>
                <th class="">公告标题</th><th>公告摘要</th><th >发布时间</th><th class="table-set">操作</th>
              </tr>
              </thead>
              <tbody>
              <%
              	List<Notice> pmAll = (List<Notice>) request.getAttribute("pmAll");
              	if(pmAll == null ||pmAll.size() == 0) {
              		%>
              		<p style="font-size:22px;">暂时没有公告！</p>
              		<%
              	} else {
              		for(Notice nt:pmAll) {
              		%>
              		<tr>
		                <td><a href="javascript:;" onclick="show_content('scontent_<%=nt.getNt_id() %>')"><%=nt.getNt_title() %></a>
		                <span id="scontent_<%=nt.getNt_id() %>" style="display:none;"><%=nt.getNt_content() %></span>
		                </td>
		                <td><%=nt.getNt_desc() %></td>
		                <td class="am-hide-sm-only"><%
		                //注册时间转换
		                DataTypeConver dtc = new DataTypeConver();
		                out.print(dtc.timeToString(nt.getNt_time(),"yyyy-MM-dd HH:mm:ss"));
		                %></td>
		                <td>
		                  <div class="am-btn-toolbar">
		                    <div class="am-btn-group am-btn-group-xs">
		                    <% if(request.getSession().getAttribute("super_type") != null) { %>
		                      <a class="am-btn am-btn-default am-btn-xs am-text-secondary" onclick="location.href='<%=pagePath %>/webadmin/notice/edit?nt_id=<%=nt.getNt_id() %>'"><span class="am-icon-pencil-square-o"></span> 编辑公告</a>
		                      <a class="am-btn am-btn-default am-btn-xs am-text-danger am-hide-sm-only" href="javascript:;" onclick="delete_partment(<%=nt.getNt_id() %>);"><span class="am-icon-trash-o"> 删除公告</span></a>
		                      <% } %>
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
              共 <%=pmAll.size() %> 个公告
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

function delete_partment(nt_id) {
	
	
	layer.confirm('您确定要删除吗？删除之后不能恢复！', {
	  btn: ['确定','取消'] //按钮
	}, function(){
		//加载提示 start
		var index = layer.load(0, { shade: [0.3,'#fff'] //0.1透明度的白色背景
			});
		//执行删除
		$.ajax({
            type:"post",
            async:true,
            url:"<%=pagePath %>/webadmin/notice/del.htm",
            //dataType:"html",
            data:{"nt_id":nt_id},
            success:function(data) {
            	//请求成功 关闭loding
	            layer.closeAll('loading');
              
              if(data == "success") {
            	  layer.msg("删除成功！");
            	  setTimeout("location.reload()", 2000);
            	  return true;
              }
              if(data == "filed_error") {
            	  layer.msg("删除失败！请稍后再试....");
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
	}, function(){
		layer.msg("您未选择删除！");
		return false;
	});
	return;
	
	if(confirm("您确定要删除吗？删除之后不能恢复！")) {
		//执行删除
		$.ajax({
            type:"post",
            async:true,
            url:"<%=pagePath %>/webadmin/partment/edit",
            //dataType:"html",
            data:{"pm_id":pm_id,"action":"delete"},
            success:function(data) {
              if(data == "error") {
            	  alert("参数错误！");
            	  return false;
              }
              if(data == "filed") {
            	  alert("该部门下有员工！无法删除");
            	  return false;
              }
              if(data == "success") {
            	  alert("删除成功！");
            	  return true;
              }
              if(data == "filed_error") {
            	  alert("删除失败！请稍后再试....");
            	  return false;
              }
              return;
            },
            error:function() {
            	alert("抱歉，请求失败！请稍后再试....");
            	return;
            }
          });
	} else {
		alert("您未选择删除！");
		return false;
	}
}

function show_content(id_box) {
	var scontent = $("#"+id_box).html();
	//页面层
	layer.open({
	  type: 1,
	  skin: 'layui-layer-lan', //加上边框
	  area: ['680px', '480px'], //宽高
	  content: "&nbsp;&nbsp;&nbsp;&nbsp;" + scontent
	});
}

</script>

</body>
</html>
