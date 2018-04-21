package algorithm;

import java.math.BigDecimal;

/**
 * 常用的数据计算方法类
 * @author zengzhiying
 *
 */

public class DataCompute {
	
	/*
	 * 无限大精度数字加法计算
	 */
	public static String bigNumberAdd(String s1, String s2) {
		BigDecimal a1 = new BigDecimal(s1);
		BigDecimal a2 = new BigDecimal(s2);
		return (a1.add(a2)).toString();
	}
	
	/*
	 * 无限大数字乘法计算
	 */
	public static String bigNumberMultiply(String s1, String s2) {
		BigDecimal a1 = new BigDecimal(s1);
		BigDecimal a2 = new BigDecimal(s2);
		return a1.multiply(a2).toString();
	}
	
	
	/*
	 * 计算一个正整数的位数
	 */
	public static int intDigit(int num) {
		final int ERROR = -1;
		int count = 0;
		if(num >= 0 && num <= Integer.MAX_VALUE) {
			
			while(num != 0) {
				num/=10;
				count++;
			}
			
			return count;
		} else {
			return ERROR;
		}
	}
	
	/*
	 * 质数判断的方法
	 */
	public static boolean isPrimeNumber(int num) {
		if(num == 2) {
			return true;
		}
		if(num%2 == 0) {
			return false;
		}
		int sqrt_num = (int) Math.sqrt((double)num);
		boolean flag = true;
		for(int i = 3;i <= sqrt_num;i+=2) {
			if(num%i == 0) {
				flag = false;
			}
		}
		return flag;
	}
	
	/*
	 * 计算一个数的绝对值
	 */
	public static double abs(double x) {
		if(x < 0) {
			return -x;
		} else {
			return x;
		}
	}
	
	/**
	 * 计算平方根 牛顿迭代法
	 */
	public static double sqrt(double a) {
		if(a < 0) {
			return Double.NaN;
		}
		double err = 1e-15;
		double t = a;
		while(Math.abs(t - a/t) > err*t) {
			t = (a/t + t)/2.0;
		}
		return t;
	}
	
	/**
	 * 计算指教三角形的斜边
	 */
	public static double hypotenuse(double a, double b) {
		return Math.sqrt(a*a + b*b);
	}
	
	/**
	 * 计算二级调和级数
	 */
	public static double harmonic(int n) {
		double sum = 0.0;
		for(int i = 1; i <= n; i++) {
			sum += 1.0/i;
		}
		return sum;
	}
	
	/**
	 * 计算一个正整数阶乘
	 */
	public static String factorial(int n) {
		String result = "1";
		for(int i = 1;i <= n;i++) {
			result = bigNumberMultiply(result, Integer.toString(i));
		}
		return result;
	}
	
	/**
	 * 判断一个整数字符串的位数，并转换为科学超过一定位数用科学计数法表示
	 */
	public static String numberString(String numstr) {
		//定义保留的小数位数
		int n = 15;
		if(numstr.length() <= (n + 1)) {
			return numstr;
		} else {
			
			String result = numstr.substring(0, 1) + "." + numstr.substring(1, n + 1) + "e+" + (numstr.length() - 1);
			return result;
		}
	}
	
	public static void main(String[] args) {
		System.out.println(hypotenuse(3, 4));
		System.out.println(sqrt(2));
		System.out.println(numberString(factorial(98)));
	}
	
}
