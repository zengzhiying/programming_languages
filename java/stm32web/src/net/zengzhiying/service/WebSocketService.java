package net.zengzhiying.service;

import java.io.IOException;

import javax.websocket.OnClose;
import javax.websocket.OnMessage;
import javax.websocket.OnOpen;
import javax.websocket.Session;
import javax.websocket.server.ServerEndpoint;

import net.zengzhiying.beans.Message;

@ServerEndpoint("/stm_websocket")
public class WebSocketService {
	
	@OnMessage
	public void onMessage(String message, Session session) {
		//获得web客户端的数据
		System.out.println("data: " + message);
		//原始静态变量
		String sendData = Message.getData();
		//最新静态变量
		String newData;
		
		try {
			//初始化输出
			if(sendData != null) {
				session.getBasicRemote().sendText(sendData);
			}
			int i = 1;
			System.out.println("进入循环...");
			while(true) {
				newData = Message.getData();
				if(newData != null && !newData.equals(sendData)) {
					session.getBasicRemote().sendText(Message.getData());
					i++;
					System.out.println("发送数据：" + i + "条！");
				}
				sendData = newData;
				Thread.sleep(1000);
			}
		} catch (IOException e) {
			e.printStackTrace();
		} catch (InterruptedException e) {
			e.printStackTrace();
		}
		
		System.out.println("一个客户端断开，程序退出循环...");
		
	}
	
	
	@OnOpen
	public void onOpen() {
		System.out.println("Client connected...");
	}
	
	@OnClose
	public void onClose() {
		System.out.println("Connection closed !");
	}
	
}
