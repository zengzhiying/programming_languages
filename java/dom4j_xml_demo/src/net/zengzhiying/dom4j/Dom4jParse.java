package net.zengzhiying.dom4j;

import java.io.File;
import java.util.Iterator;
import java.util.List;

import org.dom4j.Attribute;
import org.dom4j.Document;
import org.dom4j.DocumentException;
import org.dom4j.Element;
import org.dom4j.io.SAXReader;

public class Dom4jParse {

    public static void main(String[] args) throws DocumentException {
        // DOM4J解析books.xml文件
        String filename = "books.xml";
        Dom4jParse dj = new Dom4jParse();
        dj.dom4jxml(filename);
    }
    
    private void dom4jxml(String filename) throws DocumentException {
        // 创建SAXReader对象
        SAXReader reader = new SAXReader();
        // 加载xml文件获取document对象
        Document document = reader.read(new File(filename));
        // 获取根节点bookstore  类型:Element
        Element bookstore = document.getRootElement();
        // 通过下面方法获取迭代器，取得根节点下所有对象
        Iterator<?> it = bookstore.elementIterator();
        // 遍历迭代器，获取数据
        while(it.hasNext()) {
            System.out.println("---开始遍历---");
            // 获取book结点
            Element book = (Element) it.next();
            // 获取book属性
            @SuppressWarnings("unchecked")
            List<Attribute> bookattr = book.attributes();
            for(Attribute attr:bookattr) {
                System.out.println("属性名：" + attr.getName()
                + " 属性值：" + attr.getValue());
            }
            //获取book子节点下的信息
            Iterator<?> itt = book.elementIterator();
            while(itt.hasNext()) {
                Element ittr = (Element) itt.next();
                System.out.println("节点名：" + ittr.getName()
                + "节点值：" + ittr.getStringValue());
            }
            System.out.println("---结束遍历---");
        }
    }
}
