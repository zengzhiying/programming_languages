package net.zengzhiying;

import com.corundumstudio.socketio.Configuration;
import com.corundumstudio.socketio.SocketIONamespace;
import com.corundumstudio.socketio.SocketIOServer;

/**
 * 主动推送
 * @author monchickey
 *
 */

public class RunningServer2 {
    public static void main(String[] args) throws InterruptedException {
        Configuration config = new Configuration();
        config.setHostname("192.168.110.130");
        config.setPort(9092);
        final SocketIOServer server = new SocketIOServer(config); 
        server.start();
        String uid = "server1";
        String namespace = String.format("/%s_%s", "msg", uid);//构建命名空间
        SocketIONamespace namespace1 = server.addNamespace(namespace); //设置命名空间

        for (int i = 0; i < 50; i++) { 
            Thread.sleep(2000); 
            namespace1.getBroadcastOperations().sendEvent("message", 1); //每次发送数字一
        }
        Thread.sleep(Integer.MAX_VALUE);
        server.stop();
    }
}
