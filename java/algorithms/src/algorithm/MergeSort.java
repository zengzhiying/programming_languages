package algorithm;

/**
 * 归并排序
 * @author zengzhiying
 *
 */

public class MergeSort {
	
	/**
	 * 归并排序辅助方法 - 数组归并
	 * @param a
	 * @param low
	 * @param mid
	 * @param high
	 */
	private static void merge(double[] a, int low, int mid, int high) {
		int i = low;
		int j = mid + 1;
		int k = 0;
		double[] a2 = new double[high - low + 1];
		while(i <= mid && j <= high) {
			if(a[i] <= a[j]) {
				a2[k] = a[i];
				i++;
				k++;
			} else {
				a2[k] = a[j];
				j++;
				k++;
			}
		}
		
		while(i <= mid) {
			a2[k] = a[i];
			i++;
			k++;
		}
		while(j <= high) {
			a2[k] = a[j];
			j++;
			k++;
		}
		
		for(k = 0, i = low;i <= high;i++,k++) {
			a[i] = a2[k];
		}
	}
	
	/**
	 * 归并排序辅助方法 - 子表归并
	 * @param a
	 * @param gap
	 * @param length
	 */
	private static void mergePass(double[] a, int gap, int length) {
		int i;
		
		for(i = 0;i + 2*gap - 1 < length;i = i + 2*gap) {
			merge(a, i, i + gap - 1, i + 2*gap - 1);
		}
		
		if(i + gap - 1 < length) {
			merge(a, i, i + gap - 1, length - 1);
		}
	}
	
	/**
	 * 归并排序主入口方法
	 * 排序稳定
	 * O(nlogn)
	 * @param a
	 * @return
	 */
	public static double[] sort(double[] a) {
		int n = a.length;
		for(int gap = 1;gap < n;gap *= 2) {
			mergePass(a, gap, n);
		}
		return a;
	}
	
	public static void main(String[] args) {
		double[] a = {12, 24, 23,38,19,29,29.3,26.5,11.56,398.2,10.678};
		double[] b = sort(a);
		for(double b1:b) {
			System.out.print(b1 + " ");
		}
	}
	
}
