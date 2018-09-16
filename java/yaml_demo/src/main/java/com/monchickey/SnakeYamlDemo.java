package com.monchickey;

import org.yaml.snakeyaml.Yaml;
import org.yaml.snakeyaml.constructor.Constructor;

import java.io.*;
import java.util.List;
import java.util.Map;

/**
 * 建议生产环境中使用
 */
public class SnakeYamlDemo {
    public static void main(String[] args) {
        InputStream input = null;
        InputStream input2 = null;
        try {
            // yaml转map等结构
            input = new FileInputStream(new File("config.yaml"));
            Yaml yaml = new Yaml();
            Map m = yaml.load(input);
            System.out.println("host: " + m.get("host").toString());
            String hbaseHost = ((Map) m.get("hbase")).get("host").toString();
            int hbasePort = Integer.valueOf(((Map) m.get("hbase")).get("port").toString());
            System.out.println("hbase host: " + hbaseHost + ", port: " + hbasePort);
            // list配置
            List l = (List) m.get("hosts1");
            List l1 = (List) m.get("hosts2");
            System.out.println(l + ", " + l1);

            // yaml转java bean
            input2 = new FileInputStream(new File("config_bean.yaml"));
            Yaml yaml1 = new Yaml(new Constructor(ConfigBean.class));
            ConfigBean configBean = yaml1.load(input2);
            System.out.println(configBean.toString());
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        } finally {
            if(input != null) {
                try {
                    input.close();
                    input2.close();
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        }
    }
}
