package net.zengzhiying.beans;

/**
 * 统计某个部门对应的人数(除部门经理外)
 * @author zengzhiying
 *
 */

public class PartToNumber {
	
	private UserToPart utp;
	private int number;
	
	public UserToPart getUtp() {
		return utp;
	}
	public void setUtp(UserToPart utp) {
		this.utp = utp;
	}
	public int getNumber() {
		return number;
	}
	public void setNumber(int number) {
		this.number = number;
	}
	
}
