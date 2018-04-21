package algorithm;

/**
 * 插入排序
 * @author code more than
 */

public class InsertionSort {
	
	/**
	 * 插入排序 - 直接插入排序
	 * 升序排列算法    插入排序是稳定的
	 * 平均时间复杂度 O(n^2) 不适用于大量数据的排序
	 * @param a
	 * @return
	 */
	public static double[] sort(double[] a) {
		int N = a.length;
		for(int i = 1;i < N;i++) {
			for(int j = i;j > 0 && (a[j] < a[j-1]);j--) {
				double temp = a[j];
				a[j] = a[j-1];
				a[j-1] = temp;
			}
		}
		return a;
	}
	
	/**
	 * 希尔排序 - 插入排序的一种改进 - 升序排序
	 * 希尔排序是不稳定的
	 * 平均时间复杂度O(n^1.3)
	 * @param a
	 * @return
	 */
	public static double[] shellSort(double[] a) {
		int N = a.length;
		int h = 1;
		while(h < N/3) {
			h = 3*h + 1;
		}
		while(h >= 1) {
			//将数组变的有序
			for(int i = h;i < N;i++) {
				for(int j = i;j >= h && (a[j] < a[j-1]);j -= h) {
					double temp = a[j];
					a[j] = a[j-1];
					a[j-1] = temp;
				}
			}
			h = h/3;
		}
		return a;
	}
	
	public static void main(String[] args) {
		double[] a = {1.2, 2.3, 1.8, 6.0, 3.2, 7.4, 1.6, 3.9, 15.6, 12.3, 27.6, 23.5};
		a = shellSort(a);
		for(double an:a) {
			System.out.print(an + " ");
		}
	}
}
