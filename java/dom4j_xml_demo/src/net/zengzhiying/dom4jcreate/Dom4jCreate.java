package net.zengzhiying.dom4jcreate;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;

import org.dom4j.Document;
import org.dom4j.DocumentHelper;
import org.dom4j.Element;
import org.dom4j.io.OutputFormat;
import org.dom4j.io.XMLWriter;

/**
 * xml文档创建示例 rss订阅 rss属于xml的一种应用
 * @author zengzhiying
 *
 */

public class Dom4jCreate {

    public static void main(String[] args) throws IOException {
        Dom4jCreate doc = new Dom4jCreate();
        doc.dom4jcreate();
    }
    private void dom4jcreate() throws IOException {
        //创建document对象
        Document document = DocumentHelper.createDocument();
        //创建根节点rss
        Element rss = document.addElement("rss");
        //向rss节点中添加版本属性
        rss.addAttribute("version", "2.0");
        
        //生成子节点和节点内容
        Element channel = rss.addElement("channel");
        Element title = channel.addElement("title");
        // 添加属性值
        title.addAttribute("name", "国内新闻");
        title.addElement("time").setText("2016-12-26 06:06:00");
        Element content = title.addElement("content");
        //特殊字符自动转义解决
        content.setText("这是一篇国内新闻<br />");
        //设置生成xml的格式
        OutputFormat format = OutputFormat.createPrettyPrint();
        //设置编码
        format.setEncoding("utf-8");
        
        //生成xml文件
        XMLWriter writer = new XMLWriter(new FileOutputStream(new File("rss_news.xml")),format);
        //设置不转义，默认转义
        writer.setEscapeText(false);
        writer.write(document);
        writer.close();
    }
}
