package net.zengzhiying.beans;

/**
 * User和Partment关联实体类
 * @author zengzhiying
 *
 */

public class UserToPart {
	
	private User user;
	private Partment pm;
	
	public UserToPart() {
		
	}
	
	public UserToPart(User user, Partment pm) {
		this.user = user;
		this.pm = pm;
	}
	
	public User getUser() {
		return user;
	}
	public void setUser(User user) {
		this.user = user;
	}
	public Partment getPm() {
		return pm;
	}
	public void setPm(Partment pm) {
		this.pm = pm;
	}
	
	
}
