package net.zengzhiying.client;

import java.text.DecimalFormat;
import java.util.Random;

import coodog.filehandle.FileIO;

public class FileTest {
	
	private static boolean vtel;
	private static int num = 100;
	
	public static void main(String[] args) {
		//循环生成随机温度数据
		vtel = true;
		int i = 0;
		while(vtel) {
			FileIO fi = new FileIO();
			DecimalFormat df = new DecimalFormat("##0.00");
			fi.fileWrite("data/stm32.data", df.format(Math.random()*50) + " " + df.format(Math.random()*40));
			//延时
			try {
				Thread.sleep(2000);
			} catch (InterruptedException e) {
				e.printStackTrace();
			}
			//i++;
			if(i > num) {
				break;
			}
		}
		//System.out.println(df.format(Math.random()*50));
	}
	
}
