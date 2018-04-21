package net.zengzhiying;

import com.corundumstudio.socketio.Configuration;
import com.corundumstudio.socketio.SocketIOServer;

/**
 * 被动回复
 * @author monchickey
 *
 */

public class RunningServer 
{
    public static void main(String[] args) throws InterruptedException {
        Configuration configuration = new Configuration();
        configuration.setHostname("192.168.110.130");
        configuration.setPort(9023);
        SocketIOServer server = new SocketIOServer(configuration);
        MsgEventListener listener = new MsgEventListener(server);
        server.addEventListener("message", String.class, listener);
        // 启动服务
        server.start();
        System.out.println("server start...");
        Thread.sleep(Integer.MAX_VALUE);
        server.stop();
    }
}
