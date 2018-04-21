package net.zengzhiying.service;

import coodog.filehandle.FileIO;

public class CalcThread extends Thread {
	
	private String text;
	
	public CalcThread(String text) {
		this.text = text;
	}
	
	public void run() {
		try {
			//延时处理
			sleep(100000);
			
			//写文件
			FileIO fi = new FileIO();
			if(fi.charFileWrite("D:\\abcd.out", "您输入的内容是：" + text)) {
				//写入成功
				System.out.println("线程终止...");
			}
		} catch (InterruptedException e) {
			e.printStackTrace();
		}
	}
	
}
