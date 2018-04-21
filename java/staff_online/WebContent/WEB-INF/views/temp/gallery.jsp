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
  <title>Amaze UI Admin gallery Examples</title>
  <meta name="description" content="这是一个 gallery 页面">
  <meta name="keywords" content="gallery">
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
          <strong class="am-text-primary am-text-lg">相册</strong> / <small>Gallery</small>
        </div>
      </div>

      <hr>

      <ul class="am-avg-sm-2 am-avg-md-4 am-avg-lg-6 am-margin gallery-list">
        <li>
          <a href="#">
            <img class="am-img-thumbnail am-img-bdrs" src="http://s.amazeui.org/media/i/demos/bing-1.jpg" alt=""/>
            <div class="gallery-title">远方 有一个地方 那里种有我们的梦想</div>
            <div class="gallery-desc">2375-09-26</div>
          </a>
        </li>
        <li>
          <a href="#">
            <img class="am-img-thumbnail am-img-bdrs" src="http://s.amazeui.org/media/i/demos/bing-2.jpg" alt=""/>
            <div class="gallery-title">君可见漫天落霞</div>
            <div class="gallery-desc">2375-09-26</div>
          </a>
        </li>
        <li>
          <a href="#">
            <img class="am-img-thumbnail am-img-bdrs" src="http://s.amazeui.org/media/i/demos/bing-3.jpg" alt=""/>
            <div class="gallery-title">此刻鲜花满天幸福在身边</div>
            <div class="gallery-desc">2375-09-26</div>
          </a>
        </li>
        <li>
          <a href="#">
            <img class="am-img-thumbnail am-img-bdrs" src="http://s.amazeui.org/media/i/demos/bing-4.jpg" alt=""/>
            <div class="gallery-title">你当我是浮夸吧</div>
            <div class="gallery-desc">2375-09-26</div>
          </a>
        </li>
        <li>
          <a href="#">
            <img class="am-img-thumbnail am-img-bdrs" src="http://s.amazeui.org/media/i/demos/bing-4.jpg" alt=""/>
            <div class="gallery-title">远方 有一个地方 那里种有我们的梦想</div>
            <div class="gallery-desc">2375-09-26</div>
          </a>
        </li>
        <li>
          <a href="#">
            <img class="am-img-thumbnail am-img-bdrs" src="http://s.amazeui.org/media/i/demos/bing-1.jpg" alt=""/>
            <div class="gallery-title">斜阳染幽草 几度飞红</div>
            <div class="gallery-desc">2375-09-26</div>
          </a>
        </li>
        <li>
          <a href="#">
            <img class="am-img-thumbnail am-img-bdrs" src="http://s.amazeui.org/media/i/demos/bing-1.jpg" alt=""/>
            <div class="gallery-title">远方 有一个地方 那里种有我们的梦想</div>
            <div class="gallery-desc">2375-09-26</div>
          </a>
        </li>
        <li>
          <a href="#">
            <img class="am-img-thumbnail am-img-bdrs" src="http://s.amazeui.org/media/i/demos/bing-3.jpg" alt=""/>
            <div class="gallery-title">你当我是浮夸吧</div>
            <div class="gallery-desc">2375-09-26</div>
          </a>
        </li>
        <li>
          <a href="#">
            <img class="am-img-thumbnail am-img-bdrs" src="http://s.amazeui.org/media/i/demos/bing-3.jpg" alt=""/>
            <div class="gallery-title">远方 有一个地方 那里种有我们的梦想</div>
            <div class="gallery-desc">2375-09-26</div>
          </a>
        </li>
        <li>
          <a href="#">
            <img class="am-img-thumbnail am-img-bdrs" src="http://s.amazeui.org/media/i/demos/bing-2.jpg" alt=""/>
            <div class="gallery-title">君可见漫天落霞</div>
            <div class="gallery-desc">2375-09-26</div>
          </a>
        </li>
        <li>
          <a href="#">
            <img class="am-img-thumbnail am-img-bdrs" src="http://s.amazeui.org/media/i/demos/bing-3.jpg" alt=""/>
            <div class="gallery-title">你当我是浮夸吧</div>
            <div class="gallery-desc">2375-09-26</div>
          </a>
        </li>
        <li>
          <a href="#">
            <img class="am-img-thumbnail am-img-bdrs" src="http://s.amazeui.org/media/i/demos/bing-2.jpg" alt=""/>
            <div class="gallery-title">君可见漫天落霞</div>
            <div class="gallery-desc">2375-09-26</div>
          </a>
        </li>
        <li>
          <a href="#">
            <img class="am-img-thumbnail am-img-bdrs" src="http://s.amazeui.org/media/i/demos/bing-1.jpg" alt=""/>
            <div class="gallery-title">斜阳染幽草 几度飞红</div>
            <div class="gallery-desc">2375-09-26</div>
          </a>
        </li>
      </ul>

      <div class="am-margin am-cf">
        <hr/>
        <p class="am-fl">共 15 条记录</p>
        <ol class="am-pagination am-fr">
          <li class="am-disabled"><a href="#">&laquo;</a></li>
          <li class="am-active"><a href="#">1</a></li>
          <li><a href="#">2</a></li>
          <li><a href="#">3</a></li>
          <li><a href="#">4</a></li>
          <li><a href="#">5</a></li>
          <li><a href="#">&raquo;</a></li>
        </ol>
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
</body>
</html>
