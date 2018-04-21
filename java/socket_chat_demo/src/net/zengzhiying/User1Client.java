package net.zengzhiying;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.io.PrintWriter;
import java.net.Socket;
import java.net.UnknownHostException;
import java.util.Scanner;

/**
 * 模仿用户1 - 客户端
 * @author zengzhiying
 *
 */

public class User1Client {
    public static void main(String[] args) {
        // 创建客户端 socket
        Socket socket = null;
        try {
            socket = new Socket("192.168.28.128", 8881);
            // 创建输出
            OutputStream os = socket.getOutputStream();
            PrintWriter out = new PrintWriter(os);
            Scanner input = new Scanner(System.in);
            while(true) {
                // 接收用户输入
                System.out.print("Please input: ");
                
                String message = input.next();
                // 发送消息到User2
                if(message.equals("close")) {
                    // 关闭会话
                    System.out.println("即将关闭...");
                    out.write("close" + "\n");
                    out.flush();
                    break;
                }
                out.write(message + "\n");
                out.flush();
                // System.out.println("send message ok...");
                
                // 接收User2回复消息
                InputStream is = socket.getInputStream();
                BufferedReader reader = new BufferedReader(new InputStreamReader(is));
                String info;
                if((info = reader.readLine()) != null) {
                    // 这里防止socket关闭而引起下一行read异常
                    if(info.equals("close")) {
                        System.out.println("对方User2关闭了会话...");
                        break;
                    }
                    System.out.println("from User2: " + info);
                }
                
                
            }
            // 关闭输入
            input.close();
            
        } catch (UnknownHostException e) {
            System.out.println("主机不存在...");
            e.printStackTrace();
        } catch (IOException e) {
            System.out.println("创建socket失败...");
            e.printStackTrace();
        } finally {
            System.out.println("程序终止退出...");
            if(socket != null) {
                try {
                    // 关闭输出流
                    socket.shutdownOutput();
                    socket.close();
                } catch (IOException e) {
                    System.out.println("关闭socket...失败...");
                    e.printStackTrace();
                }
            }
        }
    }
}
