package algorithm;

/**
 * 最大子数组
 * 对于输入一个整数数组，数组中既有正数也有负数，数组中连续一个或多个整数会组成一个子数组，每个子数组都有一个和，所有子数组和的值最大的数组是最大子数组
 * 子数组的个数为O(n^2)
 * @author zengzhiying
 *
 */

public class MaxSubArray {
	
	/**
	 * 求某一数组最大子数组的和 暴力求解法
	 * @param a
	 * @return
	 */
	public static int maxSubArraySum(int[] a) {
		int i,j;
		int length = a.length;
		int maxSum = 0;
		for(i = 0; i < length;i++) {
			int curSum = 0;
			//累加循环
			for(j = i;j < length;j++) {
				curSum += a[j];
				if(curSum > maxSum) {
					maxSum = curSum;
				}
			}
		}
		return maxSum;
	}
	
	/**
	 * 线性时间算法求解最大子数组的和
	 * @param a
	 * @return
	 */
	public static int maxSubArraySum1(int[] a) {
		int i;
		int maxSum = 0;
		int curSum = 0;
		int len = a.length;
		for(i = 0;i < len;i++) {
			curSum += a[i];
			if(curSum > maxSum) {
				maxSum = curSum;
			}
			if(curSum < 0) {
				curSum = 0;
			}
		}
		return maxSum;
	}
	
	public static void main(String[] args) {
		int[] a = {10,15,-100,3,-2,-6,6,10,20};
		System.out.println(maxSubArraySum1(a));
	}
	
}
