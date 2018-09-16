package com.monchickey;

public class ConfigBean {
    private String host;
    private int number1;

    public String getHost() {
        return host;
    }

    public void setHost(String host) {
        this.host = host;
    }

    public int getNumber1() {
        return number1;
    }

    public void setNumber1(int number1) {
        this.number1 = number1;
    }

    @Override
    public String toString() {
        return "ConfigBean{" +
                "host='" + host + '\'' +
                ", number1=" + number1 +
                '}';
    }
}
