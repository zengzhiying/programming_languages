package net.zengzhiying.beans;

/**
 * 部门映射实体类
 * @author zengzhiying
 *
 */

public class Partment {
	
	private int pm_id;
	private String pm_name;
	private String pm_describe;
	private long setup_time;
	public int getPm_id() {
		return pm_id;
	}
	public void setPm_id(int pm_id) {
		this.pm_id = pm_id;
	}
	public String getPm_name() {
		return pm_name;
	}
	public void setPm_name(String pm_name) {
		this.pm_name = pm_name;
	}
	public String getPm_describe() {
		return pm_describe;
	}
	public void setPm_describe(String pm_describe) {
		this.pm_describe = pm_describe;
	}
	public long getSetup_time() {
		return setup_time;
	}
	public void setSetup_time(long setup_time) {
		this.setup_time = setup_time;
	}
	@Override
	public String toString() {
		return "Partment [pm_id=" + pm_id + ", pm_name=" + pm_name + ", pm_describe=" + pm_describe + ", setup_time="
				+ setup_time + "]";
	}
	
	
	
}
