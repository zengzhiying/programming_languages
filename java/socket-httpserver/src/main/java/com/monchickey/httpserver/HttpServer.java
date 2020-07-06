package com.monchickey.httpserver;

import java.io.*;
import java.net.InetAddress;
import java.net.ServerSocket;
import java.net.Socket;
import java.util.ArrayList;

/**
 * java -cp .:httpserver-1.0-SNAPSHOT.jar: com.monchickey.httpserver.HttpServer
 */
public class HttpServer {

    /**
     * 获取路由和url参数
     * @param reqLine
     * @return
     */
    private static String[] getRouterArgs(String reqLine) {
        String[] routeArgs = new String[]{"", ""};
        String[] args = reqLine.split(" ");
        if(args.length == 3) {
            int index = args[1].indexOf("?");
            if(index != -1) {
                routeArgs[0] = args[1].substring(0, index);
                routeArgs[1] = args[1].substring(index + 1);
            } else {
                routeArgs[0] = args[1];
            }
        }
        return routeArgs;
    }

    public static void main(String[] args) throws InterruptedException {
        try {
            // InetAddress.getByAddress(new byte[]{127, 0, 0, 1})
            // InetAddress.getByName("192.168.0.31")
            ServerSocket serverSocket = new ServerSocket(8888, 128);
            System.out.println(HandlerQueue.queue.size());
            HandlerThreadPool pool = new HandlerThreadPool(32);
            pool.runningPool();
            while (true) {
                Socket sock = serverSocket.accept();
                InputStream inputStream = sock.getInputStream();
                BufferedReader reader = new BufferedReader(new InputStreamReader(inputStream));
                int contentLength = 0;
                String method = "";
                String router = "";
                String params = "";
                String data = "";
                char[] buf = new char[1024];
                int size = 0;
                StringBuffer sb = new StringBuffer();
                do {
                    size = reader.read(buf);
                    if(size < buf.length && size > 0) {
                        char[] tmp = new char[size];
                        System.arraycopy(buf, 0, tmp, 0, size);
                        sb.append(tmp);
                    } else {
                        sb.append(buf);
                    }
                } while (size == buf.length);
                String requestContent = sb.toString();
                int contextIndex = requestContent.indexOf("\r\n\r\n");
                if(contextIndex == -1) {
                    System.out.println("http请求格式错误!");
                } else {
                    String headers = requestContent.substring(0, contextIndex);
                    data = requestContent.substring(contextIndex + 4);
                    String[] headerLines = headers.split("\r\n");
                    for(String headerLine: headerLines) {
                        if(headerLine.startsWith("GET")) {
                            method = "GET";
                            String[] routerArgs = getRouterArgs(headerLine);
                            router = routerArgs[0];
                            params = routerArgs[1];
                        }

                        if(headerLine.startsWith("POST")) {
                            method = "POST";
                            String[] routerArgs = getRouterArgs(headerLine);
                            router = routerArgs[0];
                            params = routerArgs[1];
                        }
                        if(headerLine.startsWith("Content-Length")) {
                            String length = headerLine.split(":")[1].trim();
                            contentLength = Integer.parseInt(length);
                            System.out.println("Content-Length: " + contentLength);
                        }

                    }
                }
                // System.out.println("method: " + method + " route: " + router);
                // System.out.println("args: " + params);
                // System.out.println("body: " + data);
                if(!method.equals("") && !router.equals("")) {
                    HttpHandler handler = new HttpHandler(sock, method, router, params, data);
                    HandlerQueue.queue.put(handler);
                    System.out.println("queue length: " + HandlerQueue.queue.size());
                }

                // reader.close();
                // inputStream.close();
                // System.out.println("request end.");
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
