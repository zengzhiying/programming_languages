package net.zengzhiying;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.io.PrintWriter;
import java.net.ServerSocket;
import java.net.Socket;
import java.util.Scanner;

/**
 * 模仿用户2 - 服务端
 * @author zengzhiying
 *
 */

public class User2Server {
    public static void main(String[] args) throws InterruptedException {
        ServerSocket serverSocket = null;
        Socket socket = null;
        try {
            serverSocket = new ServerSocket(8881);
            socket = serverSocket.accept();
            InputStream is = socket.getInputStream();
            InputStreamReader reader = new InputStreamReader(is);
            // 添加缓冲
            BufferedReader buffer = new BufferedReader(reader);
            Scanner scanner = new Scanner(System.in);
            while(true) {
                String info;
                if((info = buffer.readLine()) != null) {
                    if(info.equals("close")) {
                        System.out.println("对方User1关闭了会话...");
                        break;
                    }
                    System.out.println("from User1: " + info);
                }
                
                OutputStream os = socket.getOutputStream();
                PrintWriter out = new PrintWriter(os);
                System.out.print("Please input: ");
                String message = scanner.next();
                if(message.equals("close")) {
                    System.out.println("即将关闭会话...");
                    out.write("close");
                    out.flush();
                    break;
                }
                out.write(message + "\n");
                out.flush();
            }
            scanner.close();
        } catch (IOException e) {
            System.out.println("创建socket失败...");
            e.printStackTrace();
        } finally {
            System.out.println("程序终止退出...");
            try {
                if(socket != null) {
                    socket.shutdownInput();
                    socket.close();
                }
                if(serverSocket != null)
                    serverSocket.close();
            } catch (IOException e) {
                System.out.println("关闭socket...异常");
                e.printStackTrace();
            }
        }
    }
}
