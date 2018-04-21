package algorithm;

import libs.StdDraw;

public class DrawImage {
	
	/**
	 * 绘制函数曲线
	 */
	public static void drawFunction() {
		int n = 100;
		StdDraw.setXscale(0, n);
		StdDraw.setYscale(0, n*n);
		StdDraw.setPenRadius(.01);
		for(int i = 1;i <= n;i++) {
			StdDraw.point(i, i);
			StdDraw.point(i, i*i);
			StdDraw.point(i, i*Math.log(i));
		}
	}
	
	/**
	 * 绘制随机数组
	 */
	public static void randomArray() {
		int n = 50;
		double[] a = new double[50];
		for(int i = 0;i < n;i++) {
			a[i] = Math.random();
		}
		for(int i = 1;i < n;i++) {
			double x = 1.0*i/n;
			double y = a[i]/2.0;
			double rw = 0.5/n;
			double rh = a[i]/2.0;
			StdDraw.filledRectangle(x, y, rw, rh);
		}
	}
	
	public static void main(String[] args) {
//		drawFunction();
//		randomArray();
	}
	
}
