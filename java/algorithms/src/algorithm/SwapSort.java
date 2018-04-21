package algorithm;

/**
 * 交换排序
 * @author zengzhiying
 *
 */

public class SwapSort {
	
	/**
	 * 交换排序 - 冒泡排序
	 * 升序排序
	 * 排序稳定
	 * 时间复杂度 O(n^2)
	 * @param a
	 * @return
	 */
	public static double[] bubbleSort(double[] a) {
		int N = a.length;
		for(int i = 0; i < N - 1;i++) {
			for(int j = 0;j < N-i-1;j++) {
				if(a[j] > a[j + 1]) {
					double temp = a[j];
					a[j] = a[j + 1];
					a[j + 1] = temp;
				}
			}
		}
		return a;
	}
	
	/**
	 * 冒泡排序改进版1
	 * @param a
	 * @return
	 */
	public static double[] bubbleSortPro1(double[] a) {
		int N = a.length;
		int i = N - 1;
		while(i > 0) {
			int pos = 0;
			for(int j = 0;j < i;j++) {
				if(a[j] > a[j + 1]) {
					pos = j;
					double temp = a[j];
					a[j] = a[j + 1];
					a[j + 1] = temp;
				}
			}
			i = pos;
		}
		return a;
	}
	
	/**
	 * 冒泡排序改进版2
	 * @param a
	 * @return
	 */
	public static double[] bubbleSortPro2(double[] a) {
		int low = 0;
		int high = a.length - 1;
		int j;
		double tmp;
		while(low < high) {
			for(j = low;j < high;j++) {
				if(a[j] > a[j + 1]) {
					tmp = a[j];
					a[j] = a[j + 1];
					a[j + 1] = tmp;
				}
			}
			--high;
			for(j = high;j > low;j--) {
				if(a[j] < a[j - 1]) {
					tmp = a[j];
					a[j] = a[j - 1];
					a[j - 1] = tmp;
				}
			}
			++low;
		}
		return a;
	}
	
	/**
	 * 交换排序 - 快速排序
	 * 排序不稳定
	 * 时间复杂度 O(nlogn)
	 * @param a
	 * @return
	 */
	public static double[] quickSort(double[] a, int low, int high) {
		if(low < high) {
			int privoLoc = partition(a, low, high);
			a = quickSort(a, low, privoLoc - 1);
			a = quickSort(a, privoLoc + 1, high);
		}
		return a;
	}
	
	
	/**
	 * 快速排序辅助方法 将数列一分为二
	 * @param a
	 * @param low
	 * @param high
	 * @return
	 */
	private static int partition(double[] a,int low,int high) {
		double privotKey = a[low];
		while(low < high) {
			while(low < high && a[high] >= privotKey) {
				--high;
			}
			double temp = a[low];
			a[low] = a[high];
			a[high] = temp;
			while(low < high && a[low] <= privotKey) {
				++low;
			}
			temp = a[low];
			a[low] = a[high];
			a[high] = temp;
		}
		return low;
	}
	
	
	public static void main(String[] args) {
		double[] a = {12, 24, 23,38,19,29,29.3,26.5,11.56,398.2};
		a = quickSort(a, 0, a.length - 1);
		for(int i = 0;i < a.length;i++) {
			System.out.print(a[i] + " ");
		}
	}
	
}
