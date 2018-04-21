package net.zengzhiying.beans;

/**
 * 用户实体映射类
 * @author zengzhiying
 *
 */

public class User {
	
	private int user_id;
	private String user_name;	//员工姓名
	private int gender;	//员工性别 0女 1男
	private int age;	//年龄
	private String username;	//用户名
	private String password;	//密码
	private int department_id;	//部门id
	private int access_type;	//权限类型 1boss 2部门经理 3普通员工
	private String contact;	//联系方式
	private int is_approve;	//审批状态 0未审批 1已审批
	private long commit_time;	// 提交时间
	private long register_time;	// 转正时间
	private String user_describe;	// 备注介绍
	
	
	public int getGender() {
		return gender;
	}
	public void setGender(int gender) {
		this.gender = gender;
	}
	public int getAge() {
		return age;
	}
	public void setAge(int age) {
		this.age = age;
	}
	public String getUser_name() {
		return user_name;
	}
	public void setUser_name(String user_name) {
		this.user_name = user_name;
	}
	public int getUser_id() {
		return user_id;
	}
	public void setUser_id(int user_id) {
		this.user_id = user_id;
	}
	public String getUsername() {
		return username;
	}
	public void setUsername(String username) {
		this.username = username;
	}
	public String getPassword() {
		return password;
	}
	public void setPassword(String password) {
		this.password = password;
	}
	public int getDepartment_id() {
		return department_id;
	}
	public void setDepartment_id(int department_id) {
		this.department_id = department_id;
	}
	public int getAccess_type() {
		return access_type;
	}
	public void setAccess_type(int access_type) {
		this.access_type = access_type;
	}
	public String getContact() {
		return contact;
	}
	public void setContact(String contact) {
		this.contact = contact;
	}
	public int getIs_approve() {
		return is_approve;
	}
	public void setIs_approve(int is_approve) {
		this.is_approve = is_approve;
	}
	public long getCommit_time() {
		return commit_time;
	}
	public void setCommit_time(long commit_time) {
		this.commit_time = commit_time;
	}
	public long getRegister_time() {
		return register_time;
	}
	public void setRegister_time(long register_time) {
		this.register_time = register_time;
	}
	public String getUser_describe() {
		return user_describe;
	}
	public void setUser_describe(String user_describe) {
		this.user_describe = user_describe;
	}
	@Override
	public String toString() {
		return "User [user_id=" + user_id + ", username=" + username + ", password=" + password + ", department_id="
				+ department_id + ", access_type=" + access_type + ", contact=" + contact + ", is_approve=" + is_approve
				+ ", commit_time=" + commit_time + ", register_time=" + register_time + ", user_describe="
				+ user_describe + "]";
	}
	
	
}
