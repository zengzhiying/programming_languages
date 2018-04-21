package net.zengzhiying.beans;

/**
 * 测试实体类
 * @author zengzhiying
 *
 */
public class TestBean {
    
    public TestBean() { }
    
    public TestBean(String name, int age) {
        this.name = name;
        this.age = age;
    }
    
    private String name;
    private int age;
    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public int getAge() {
        return age;
    }

    public void setAge(int age) {
        this.age = age;
    }

    @Override
    public String toString() {
        return "TestBean [name=" + name + ", age=" + age + "]";
    }
    
    
}
