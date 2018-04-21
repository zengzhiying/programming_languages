package algorithm;

/**
 * 基数排序
 * @author zengzhiying
 *
 */

public class RadixSort {
	
	/**
	 * 基数排序 整数d表示所有数中最大数的位数 基数排序目前仅仅适用于整数
	 * 排序稳定
	 * 时间复杂度 O(d(r+n)) r为基数，d表示长度，n表示数字的个数
	 * @param a
	 * @param d
	 */
	public static void sort(int[] a, int d) {
		int k = 0;
		int n = 1;
		int m = 1;
		int length = a.length;
		int[][] temp = new int[10][length];
		int[] order = new int[10];
		while(m <= d) {
			for(int i = 0;i < length;i++) {
				int lsd = ((a[i]/n) % 10);
				temp[lsd][order[lsd]] = a[i];
				order[lsd]++;
			}
			for(int i = 0;i < 10;i++) {
				if(order[i] != 0) {
					for(int j = 0;j < order[i];j++) {
						a[k] = temp[i][j];
						k++;
					}
				}
				order[i] = 0;
			}
			n *= 10;
			k = 0;
			m++;
		}
	}
	
	/**
	 * 桶排序 没有用到比较 排序速度非常快，但是非常耗费空间资源
	 * 也是只适用于正整数排序，不要用零，会出现混乱，并且数字多大就得申请一个大约多长的数组
	 * @param a
	 * @return
	 */
	public static int[] bucketSort(int[] a, int maxNumber) {
		int[] b = new int[maxNumber + 1];
		for(int i = 0;i < a.length;i++) {
			b[a[i]] = a[i];
		}
		// 将b中非0的数据排序出来
		int j = 0;
		for(int i = 0;i < b.length;i++) {
			if(b[i] > 0) {
				a[j] = b[i];
				j++;
			}
		}
		return a;
	}
	
	
	public static void main(String[] args) {
		int[] a = {12,6,19,67,123,9,7,4,65,282};
		sort(a, 3);
		for(int a1:a) {
			System.out.print(a1 + " ");
		}
	}
}
