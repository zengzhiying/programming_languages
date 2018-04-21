package net.zengzhiying.beans;

public class Message {
	
	private int id;
	private String name;
	//数据表字段名不能是content，否则会出错
	private String password;
	
	public Message() {
		
	}
	
	public Message(int id, String name, String password) {
		this.id = id;
		this.name = name;
		this.password = password;
	}
	
	public int getId() {
		return id;
	}
	public void setId(int id) {
		this.id = id;
	}
	public String getName() {
		return name;
	}
	public void setName(String name) {
		this.name = name;
	}
	public String getPassword() {
		return password;
	}
	public void setPassword(String password) {
		this.password = password;
	}
	
	
	
}
