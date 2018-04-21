<%@ page language="java" contentType="text/html; charset=UTF-8"
    pageEncoding="UTF-8"%>
<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN" "http://www.w3.org/TR/html4/loose.dtd">
<html>
<head>
<meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
<title>stm管理界面</title>
</head>
<body>
  <div>
    <input type="submit" value="Start" onclick="start()" />
  </div>
  <div id="messages"></div>
  <script type="text/javascript">

    //初始化的时候连接建立成功
    var webSocket = new WebSocket('ws://localhost:8080/stm32web/stm_websocket');

    //启动服务端socket进程调用的方法
    function start() {
      //发送数据到web服务端
      webSocket.send('hello');
      return false;
    }

    webSocket.onerror = function(event) {
      onError(event)
    };
   
    webSocket.onopen = function(event) {
      onOpen(event)
    };
 
    webSocket.onmessage = function(event) {
      onMessage(event)
    };

    //推送消息
    function onMessage(event) {
      document.getElementById('messages').innerHTML
        += '<br />' + event.data;
    }

    //打开socket连接时执行
    function onOpen(event) {
      document.getElementById('messages').innerHTML
        = 'Connection established';
    }
 
    function onError(event) {
      alert(event.data);
    }
    
    window.onbeforeunload = function() {
      //关闭连接 服务器可能还在执行任务，所以不一定成功
  	  webSocket.close();
    }
  </script>
</body>
</html>