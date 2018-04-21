package net.zengzhiying.beans;

public class Notice {
	
	private int nt_id;
	private String nt_title;
	public String getNt_title() {
		return nt_title;
	}

	public void setNt_title(String nt_title) {
		this.nt_title = nt_title;
	}
	private String nt_desc;	//公告摘要
	private String nt_content; 	//公告内容
	private long nt_time;	//公告发布时间
	
	
	public Notice() {
		
	}
	
	public int getNt_id() {
		return nt_id;
	}
	public void setNt_id(int nt_id) {
		this.nt_id = nt_id;
	}
	public String getNt_desc() {
		return nt_desc;
	}
	public void setNt_desc(String nt_desc) {
		this.nt_desc = nt_desc;
	}
	public String getNt_content() {
		return nt_content;
	}
	public void setNt_content(String nt_content) {
		this.nt_content = nt_content;
	}
	public long getNt_time() {
		return nt_time;
	}
	public void setNt_time(long nt_time) {
		this.nt_time = nt_time;
	}
	
	
}
