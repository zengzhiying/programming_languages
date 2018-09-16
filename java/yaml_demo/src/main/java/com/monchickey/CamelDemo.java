package com.monchickey;

import com.amihaiemil.camel.Yaml;
import com.amihaiemil.camel.YamlMapping;
import com.amihaiemil.camel.YamlSequence;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.IOException;

/**
 * 测试未通过, 暂不建议使用
 */
public class CamelDemo {
    public static void main(String[] args) {
        try {
            // 读取为mapping
            YamlMapping yamlMapping = Yaml.createYamlInput(new File("config.yaml"))
                    .readYamlMapping();
            // 读取为序列 (初始加载配置不太常用)
            YamlSequence yamlSequence = Yaml.createYamlInput(new File("config.yaml"))
                    .readYamlSequence();
//            System.out.println(yamlSequence.toString());
//            System.out.println(yamlMapping);

            // host
            System.out.println("host: " + yamlMapping.string("host"));
            // hbase
            YamlMapping hbaseMapping = yamlMapping.yamlMapping("hbase");
            System.out.println("hbase host: " + hbaseMapping.string("host"));
            System.out.println("hbase port: " + Integer.valueOf(hbaseMapping.string("port")));
            System.out.println("hbase zookeeper host: " + hbaseMapping.yamlMapping("zookeeper").string("host"));
            System.out.println("hbase zookeeper port: " + hbaseMapping.yamlMapping("zookeeper").string("port"));

            // 列表
//            System.out.println("hosts1: " + yamlMapping.yamlSequence("hosts1"));

        } catch (FileNotFoundException e) {
            e.printStackTrace();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
