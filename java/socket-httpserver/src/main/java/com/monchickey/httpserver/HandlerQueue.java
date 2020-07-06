package com.monchickey.httpserver;

import java.util.concurrent.BlockingQueue;
import java.util.concurrent.LinkedBlockingQueue;

public class HandlerQueue {
    public static BlockingQueue<HttpHandler> queue = new LinkedBlockingQueue(5000);
}
