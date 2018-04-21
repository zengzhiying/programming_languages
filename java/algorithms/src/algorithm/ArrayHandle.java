package algorithm;

import java.util.Arrays;

/**
 * 常用的数组操作类
 * @author zengzhiying
 *
 */

public class ArrayHandle {
	
	/*
	 * 使用java自带的类进行数组排序
	 * 排序方式：升序
	 */
	public int[] arraysClassSort(int[] arr) {
		Arrays.sort(arr);
		return arr;
	}
	
	
	/*
	 * 求数组中的平均值
	 */
	public double averageValue(double[] arr) {
		double sum = 0.0;
		for(int i = 0;i < arr.length;i++) {
			sum += arr[i];
		}
		return sum/arr.length;
	}
	
	/*
	 * 计算数组中的最大值
	 */
	public int max(int[] num){
		if(num.length == 0){
			return 0;
		}else{
			int max = num[0];
			for(int i = 1;i < num.length;i++){
				if(num[i] > max){
					max = num[i];
				}
			}
			return max;
		}
	}
	
	/*
	 * 计算数组中的最小值
	 */
	public int min(int[] num){
		if(num.length == 0){
			return 0;
		}else{
			int min = num[0];
			for(int i = 0;i < num.length;i++){
				if(min > num[i]){
					min = num[i];
				}
			}
			return min;
		}
	}
	
	/*
	 * 复制数组，两数组之前没有任何干扰
	 */
	public double[] copyArray(double[] a) {
		int n = a.length;
		double[] b = new double[n];
		for(int i = 0;i < n;i++)
			b[i] = a[i];
		return b;
	}
	
	/*
	 * 颠倒数组中元素的顺序
	 */
	public double[] arrayReverse(double[] a) {
		int n = a.length;
		for(int i = 0;i < n/2;i++) {
			double temp = a[i];
			a[i] = a[n-1-i];
			a[n-1-i] = temp;
		}
		return a;
	}
	
	
	/*
	 * 方阵相乘，长和宽都相等的矩阵
	 */
	public double[][] arrayMulti(double[][] a, double[][] b) {
		int n = a.length;
		double[][] c = new double[n][n];
		for(int i = 0;i < n;i++) {
			for(int j = 0;j < n;j++) {
				//计算第i行和第j列的乘积
				for(int k = 0;k < n;k++) {
					c[i][j] += a[i][k] * b[k][j];
				}
			}
		}
		return c;
	}
	
	
}
