package algorithm;

/**
 * 选择排序算法
 * 首先找到数组中最小的那个元素和第一个元素交换，然后在剩下的所有元素中找到一个最小元素和第二个交换，依次类推，这种交换和输入无关，交换的次数都是N次
 */

public class SelectionSort {
	
	/**
	 * 简单选择排序 - 直接选择排序
	 * 对数组a升序排序
	 * 排序不稳定 时间复杂度O(n^2)
	 * @param a
	 * @return
	 */
	public static double[] simpleSelectionSort(double[] a) {
		int N = a.length;
		for(int i = 0;i < N - 1;i++) {
			//元素交换
			for(int j = i+1;j < N;j++) {
				if(a[j] < a[i]) {
					double temp = a[j];
					a[j] = a[i];
					a[i] = temp;
				}
			}
		}
		return a;
	}
	
	
	/**
	 * 选择排序 - 堆排序
	 * 排序不稳定 时间复杂度O(nlogn)
	 * @param a
	 * @return
	 */
	public static double[] heapSort(double[] a) {
		int N = a.length;
		buildingHeap(a, N);
		for(int i = N - 1; i > 0;--i) {
			double temp = a[i];
			a[i] = a[0];
			a[0] = temp;
			
			heapAdjust(a, 0, i);
		}
		return a;
	}
	
	/**
	 * 堆排序调整方法
	 * @param a
	 * @param s
	 * @return
	 */
	private static void heapAdjust(double[] a, int s, int N) {
		double tmp = a[s];
		int child = 2*s + 1;
		while(child < N) {
			if(child + 1 < N && a[child] < a[child + 1]) {
				++child;
			}
			if(a[s] < a[child]) {
				a[s] = a[child];
				s = child;
				child = 2*s + 1;
			} else {
				break;
			}
			a[s] = tmp;
		}
	}
	
	/**
	 * 初始堆进行调整
	 * @param a
	 * @return
	 */
	private static void buildingHeap(double[] a, int length) {
		for(int i = (length - 1)/2; i >= 0;--i) {
			heapAdjust(a, i,length);
		}
	}
	
	
	/*
	 * 测试方法
	 */
	public static void main(String[] args) {
		double[] a = {12, 24, 23,38,19,29,29.3,26.5,11.56,398.2};
		double[] b = heapSort(a);
		for(int i = 0;i < b.length;i++) {
			System.out.println(b[i]);
		}
	}
}
