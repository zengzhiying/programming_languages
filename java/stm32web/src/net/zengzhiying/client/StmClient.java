package net.zengzhiying.client;

import java.io.UnsupportedEncodingException;
import java.net.URLEncoder;

import coodog.filehandle.FileIO;
import coodog.webgrab.WebGrab;

/**
 * stm32 数据发送客户端
 * @author Administrator
 *
 */

public class StmClient {
	
	public static void main(String[] args) {
		FileIO fi = new FileIO();
		System.out.println("客户端启动...发送数据中...");
		String data;
		while(true) {
			data = fi.fileRead("data/stm32.data");
			try {
				WebGrab.httpGet("http://115.159.124.107:8080/stm32web/send", "wd=" + URLEncoder.encode(data, "utf-8"), "utf-8");
				System.out.println("发送数据：" + data);
				Thread.sleep(1000);
			} catch (UnsupportedEncodingException e) {
				e.printStackTrace();
			} catch (InterruptedException e) {
				e.printStackTrace();
			}
		}
		
	}
	
}
