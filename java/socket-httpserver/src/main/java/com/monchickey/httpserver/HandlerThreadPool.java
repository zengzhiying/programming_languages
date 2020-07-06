package com.monchickey.httpserver;

import java.io.IOException;
import java.io.OutputStream;
import java.io.PrintWriter;
import java.net.Socket;
import java.util.ArrayList;

public class HandlerThreadPool {

    private ArrayList<Thread> pool;
    private int numThread;

    public HandlerThreadPool(int numThread) {
        this.numThread = numThread;
        pool = new ArrayList<>(this.numThread);
    }

    public void runningPool() {
        for(int i = 0; i < this.numThread; i++) {
            Thread t = new Thread(new HandlerThread(i + 1));
            pool.add(0, t);
            t.start();
        }
    }

    class HandlerThread implements Runnable {

        private int threadNumber;

        public HandlerThread(int threadNumber) {
            System.out.println("Handler Thread init. " + threadNumber);
            this.threadNumber = threadNumber;
        }

        @Override
        public void run() {
            System.out.println("Handler Thread start.");
            while (true) {
                HttpHandler handler = null;
                try {
                    handler = HandlerQueue.queue.take();
                } catch (InterruptedException e) {
                    System.out.println("线程: " + threadNumber + " 读取队列异常！");
                    e.printStackTrace();
                    continue;
                }
                Socket sock = handler.getSock();
                String method = handler.getMethod();
                String router = handler.getRouter();
                String args = handler.getArgs();
                String body = handler.getBody();
                System.out.println("线程: " + threadNumber + " method: " + method +
                        " router: " + router);
                if(!args.isEmpty())
                    System.out.println("args: " + args);
                if(!body.isEmpty())
                    System.out.println("body: " + body);
                try(PrintWriter printWriter = new PrintWriter(sock.getOutputStream())) {
                    // header
                    printWriter.println("HTTP/1.1 200 OK");
                    printWriter.println("Content-Type: text/html");
                    printWriter.println("Connection: close");
                    printWriter.println();

                    printWriter.print("RECV: " + method + " " + router + "\n");
                    // 先处理再响应
                    // Thread.sleep(10000);
                } catch (IOException e) {
                    e.printStackTrace();
                }

                // 关闭socket
                try {
                    sock.close();
                } catch (IOException e) {
                    e.printStackTrace();
                }
                // 先响应再处理 可以把sock放到外部关闭效果更好, 这样可以保证队列长度个并发同时返回
                try {
                    Thread.sleep(10000);
                } catch (InterruptedException e) {
                    e.printStackTrace();
                }
            }
        }
    }
}
