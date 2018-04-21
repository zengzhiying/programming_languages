<%@ page language="java" contentType="text/html; charset=UTF-8"
    pageEncoding="UTF-8"%>
<%
	String pagePath = request.getContextPath();
%>
<div class="admin-sidebar am-offcanvas" id="admin-offcanvas">
    <div class="am-offcanvas-bar admin-offcanvas-bar">
      <ul class="am-list admin-sidebar-list">
        <li><a href="<%=pagePath %>/"><span class="am-icon-home"></span> 首页</a></li>
        <li><a href="<%=pagePath %>/webadmin/stafflist"><span class="am-icon-table"></span> 员工管理</a></li>
        <% if(request.getSession().getAttribute("super_type") != null) { %>
        <li><a href="<%=pagePath %>/webadmin/partment_manager"><span class="am-icon-pencil-square-o"></span> 部门经理管理</a></li>
        <li><a href="<%=pagePath %>/webadmin/partment"><span class="am-icon-pencil-square-o"></span> 部门管理</a></li>
        <li><a href="<%=pagePath %>/webadmin/notice.htm"><span class="am-icon-pencil-square-o"></span> 公告管理</a></li>
        <% } else { %>
        <li><a href="<%=pagePath %>/webadmin/notice.htm"><span class="am-icon-pencil-square-o"></span> 公司公告</a></li>
        <% } %>
        <li class="admin-parent">
          <a class="am-cf" data-am-collapse="{target: '#collapse-nav'}"><span class="am-icon-file"></span> 系统设置<span class="am-icon-angle-right am-fr am-margin-right"></span></a>
          <ul class="am-list am-collapse admin-sidebar-sub" id="collapse-nav" style="">
            <li><a href="<%=pagePath %>/webadmin/configs/editpasswd" class="am-cf"><span class="am-icon-check"></span> 密码修改<span class="am-icon-star am-fr am-margin-right admin-icon-yellow"></span></a></li>
            <% if(request.getSession().getAttribute("super_type") != null) { %>
            <li><a href="<%=pagePath %>/webadmin/configs/logs.view"><span class="am-icon-calendar"></span> 系统日志</a></li>
            <% } %>
          </ul>
        </li>
        
        <li class="admin-parent">
          <a class="am-cf" data-am-collapse="{target: '#collapse-nav1'}"><span class="am-icon-file"></span> 发展趋势<span class="am-icon-angle-right am-fr am-margin-right"></span></a>
          <ul class="am-list am-collapse admin-sidebar-sub" id="collapse-nav1" style="">
            <li><a href="<%=pagePath %>/webadmin/trend/income.view" target="_blank" class="am-cf"><span class="am-icon-check"></span> 收入趋势</a></li>
            <li><a href="<%=pagePath %>/webadmin/trend/produce.show" class="am-cf"><span class="am-icon-check"></span> 产出趋势</a></li>
          </ul>
        </li>
        <li><a href="<%=pagePath %>/webadmin/logout"><span class="am-icon-sign-out"></span> 注销</a></li>
      </ul>

      <div class="am-panel am-panel-default admin-sidebar-panel">
        <div class="am-panel-bd">
          <p><span class="am-icon-bookmark"></span> 公告</p>
          <p>欢迎您使用员工管理系统 —— admin</p>
        </div>
      </div>

      <div class="am-panel am-panel-default admin-sidebar-panel">
        <div class="am-panel-bd">
          <p><span class="am-icon-tag"></span> wiki</p>
          <p>Welcome to the StaffOnLine!</p>
        </div>
      </div>
    </div>
  </div>