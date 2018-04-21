package net.zengzhiying.beans;

/**
 * 员工签到数据映射类
 * @author zengzhiying
 *
 */

public class CheckWork {
	
	private int message_id;	//考勤库编号
	private int user_id;	//员工编号
	private long morn_time;	//上午签到时间 未签到是0
	private long aft_time;	//下午签到时间 未签到是0
	private String sign_date;	//签到日期 比如"20160606"
	private String sign_week;	//当前星期几
	
	
	public CheckWork() {
		
	}


	public int getMessage_id() {
		return message_id;
	}


	public void setMessage_id(int message_id) {
		this.message_id = message_id;
	}


	public int getUser_id() {
		return user_id;
	}


	public void setUser_id(int user_id) {
		this.user_id = user_id;
	}


	public long getMorn_time() {
		return morn_time;
	}


	public void setMorn_time(long morn_time) {
		this.morn_time = morn_time;
	}


	public long getAft_time() {
		return aft_time;
	}


	public void setAft_time(long aft_time) {
		this.aft_time = aft_time;
	}


	public String getSign_date() {
		return sign_date;
	}


	public void setSign_date(String sign_date) {
		this.sign_date = sign_date;
	}


	public String getSign_week() {
		return sign_week;
	}


	public void setSign_week(String sign_week) {
		this.sign_week = sign_week;
	}
	
}
