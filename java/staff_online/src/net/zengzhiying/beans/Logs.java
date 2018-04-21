package net.zengzhiying.beans;

/**
 * 日志实体类
 * @author zengzhiying
 *
 */

public class Logs {
	
	private int log_id;
	private String log_type;	//日志类型 增、删、改
	private long log_time;	//操作时间
	private String log_describe;	//日志描述
	public int getLog_id() {
		return log_id;
	}
	public void setLog_id(int log_id) {
		this.log_id = log_id;
	}
	public String getLog_type() {
		return log_type;
	}
	public void setLog_type(String log_type) {
		this.log_type = log_type;
	}
	public long getLog_time() {
		return log_time;
	}
	public void setLog_time(long log_time) {
		this.log_time = log_time;
	}
	public String getLog_describe() {
		return log_describe;
	}
	public void setLog_describe(String log_describe) {
		this.log_describe = log_describe;
	}
	
}
