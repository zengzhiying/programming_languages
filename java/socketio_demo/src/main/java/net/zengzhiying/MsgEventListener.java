package net.zengzhiying;

import com.corundumstudio.socketio.AckRequest;
import com.corundumstudio.socketio.SocketIOClient;
import com.corundumstudio.socketio.SocketIOServer;
import com.corundumstudio.socketio.listener.DataListener;

/**
 * 监听类
 *
 */
public class MsgEventListener implements DataListener<String> {

    private SocketIOServer server;

    public MsgEventListener(SocketIOServer server) {
        this.server = server;
    }

    public void onData(SocketIOClient client, String data,
            AckRequest ackSender) throws Exception {
        // chatevent为 事件的名称， data为发送的内容
        System.out.println("收到消息:" + data);
        this.server.getBroadcastOperations().sendEvent("message", data);
    }
}
