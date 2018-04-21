package net.zengzhiying;

import java.util.HashMap;
import java.util.Map;

import com.jsoniter.JsonIterator;
import com.jsoniter.any.Any;
import com.jsoniter.output.JsonStream;

import net.zengzhiying.beans.TestBean;

/**
 * json iterator库测试案例
 * @author zengzhiying
 *
 */
public class JsonIteratorDemo {
    public static void main(String[] args) {
        // Map转json串
        System.out.println("Map->json:");
        Map<String, Object> m = new HashMap<String, Object>();
        m.put("name", "zzy");
        m.put("age", 23);
        String out = JsonStream.serialize(m);
        System.out.println(out);
        // 对象转json串
        System.out.println("实体类->json:");
        TestBean tb = new TestBean("lsl", 24);
        out = JsonStream.serialize(tb);
        System.out.println(out);
        
        // json串解析
        System.out.println("json值解析:");
        String input = "{\"name\":\"test\", \"kk\": 68}";
        Any a = JsonIterator.deserialize(input);
        System.out.println(a.get("name"));
        int kk = a.get("kk").toInt();
        System.out.println(kk);
        // 解析成map
        System.out.println("json->map:");
        Map<?, ?> m2 = JsonIterator.deserialize(input, Map.class);
        System.out.println(m2.get("name"));
        
        // json串解析成对象(也可以分开赋值)
        System.out.println("json->实体类对象:");
        input = "{\"name\":\"test111\", \"age\": 23}";
        TestBean tb2 = JsonIterator.deserialize(input, TestBean.class);
        System.out.println(tb2.toString());
        
        // json 多元素解析
        System.out.println("json多元素解析和迭代:");
        input = "{\"names\":[\"test111\", \"test222\"], \"age\": [23, 27]}";
        Any anyIterator = JsonIterator.deserialize(input);
        // 获得第二个元素 也可以使用anyIterator.get("names").get(1)
        System.out.println(anyIterator.get("names", 1));
        // json 数组迭代
        Any ages = anyIterator.get("age");
        for(Any age: ages) {
            System.out.println(age.toInt());
        }
    }
}
