package net.zengzhiying.beans;

public class Message {
	
	//静态变量 为了保存一部分内存属性使用，一般不要使用 因为会涉及到共享内存的问题，要考虑并发
	private static String name;
	private static String data;
	
	public Message() {
		
	}
	
	public Message(String name, String data) {
		Message.name = name;
		Message.data = data;
	}

	public static String getName() {
		return name;
	}

	public static void setName(String name) {
		Message.name = name;
	}

	public static String getData() {
		return data;
	}

	public static void setData(String data) {
		Message.data = data;
	}
	
}
