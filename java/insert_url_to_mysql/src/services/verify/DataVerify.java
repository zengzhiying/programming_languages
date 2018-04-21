package services.verify;

import java.util.regex.Matcher;
import java.util.regex.Pattern;

/*
 * 数据验证类
 */

public class DataVerify {
	
	/*
	 * 验证字符串是否为数字
	 */
	public static boolean isNumeric(String str) {
		for(int i = 0;i < str.length();i++) {
			if(!Character.isDigit(str.charAt(i))) {
				return false;
			}
		}
		return true;
	}
	
	public static boolean isPosInt(String str) {
		Pattern pattern = Pattern.compile("^[1-9][0-9]{0,}$");
		Matcher isPos = pattern.matcher(str);
		if(!isPos.matches()) {
			return false;
		}
		return true;
	}
	
	public static void main(String[] args) {
//		if(isPosInt("02")) {
//			System.out.println("正确");
//		} else {
//			System.out.println("错误");
//		}
	}
}
