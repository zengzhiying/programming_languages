package entity;

/*
 * 实体类
 */

public class Website {
	private String title;	//网址名称
	private String link;	//网址
	private String classify;	//分类
	private int sort;
	
	public Website() {
		
	}
	
	public Website(String title, String link, String classify, int sort) {
		this.title = title;
		this.link = link;
		this.classify = classify;
		this.sort = sort;
	}

	public String getTitle() {
		return title;
	}

	public void setTitle(String title) {
		this.title = title;
	}

	public String getLink() {
		return link;
	}

	public void setLink(String link) {
		this.link = link;
	}

	public String getClassify() {
		return classify;
	}

	public void setClassify(String classify) {
		this.classify = classify;
	}

	public int getSort() {
		return sort;
	}

	public void setSort(int sort) {
		this.sort = sort;
	}
	
}
