package com.monchickey.httpserver;

import java.net.Socket;

public class HttpHandler {
    private Socket sock;
    private String method;
    private String router;
    private String args;
    private String body;

    public HttpHandler() {}

    public HttpHandler(Socket sock, String method, String router, String args, String body) {
        this.sock = sock;
        this.method = method;
        this.router = router;
        this.args = args;
        this.body = body;
    }

    public Socket getSock() {
        return sock;
    }

    public void setSock(Socket sock) {
        this.sock = sock;
    }

    public String getMethod() {
        return method;
    }

    public void setMethod(String method) {
        this.method = method;
    }

    public String getRouter() {
        return router;
    }

    public void setRouter(String router) {
        this.router = router;
    }

    public String getArgs() {
        return args;
    }

    public void setArgs(String args) {
        this.args = args;
    }

    public String getBody() {
        return body;
    }

    public void setBody(String body) {
        this.body = body;
    }

    @Override
    public String toString() {
        return "HttpHandler{" +
                "sock=" + sock +
                ", method='" + method + '\'' +
                ", router='" + router + '\'' +
                ", args='" + args + '\'' +
                ", body='" + body + '\'' +
                '}';
    }
}
