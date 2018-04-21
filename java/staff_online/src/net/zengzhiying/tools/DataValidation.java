package net.zengzhiying.tools;

import java.text.ParseException;
import java.text.SimpleDateFormat;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

/**
 * 数据验证类 保证数据的准确性
 * @author zengzhiying
 *
 */

public class DataValidation {
	
	/**
	 * 日期类型字符串验证，有日期越界判断能力，比正则更强大
	 * @param dateStr 格式必须为2016-05-30这样的格式
	 * @return
	 */
	public boolean dateStrValidation(String dateStr) {
		SimpleDateFormat format = new SimpleDateFormat("yyyy-MM-dd");
		//设置严格匹配，默认是宽松匹配
		format.setLenient(false);
		try {
			format.parse(dateStr);
			//到这里转换正常
			return true;
		} catch (ParseException e) {
			// 转换异常
			return false;
		}
	}
	
	/**
	 * 验证一个数字字符串是否为正整数
	 * @param number
	 * @return
	 */
	public boolean posInteger(String number) {
		Pattern p = Pattern.compile("^[1-9][0-9]{0,}$");
		
		Matcher m = p.matcher(number);
		if(m.find()) {
			return true;
		}
		return false;
	}
	
	/**
	 * 用户名校验
	 * 规则:4-16个字符
	 * @param username
	 * @return
	 */
	public boolean usernameValidation(String username) {
		Matcher m = Pattern.compile("^[a-zA-Z][a-zA-Z0-9_]{3,15}$").matcher(username);
		if(m.find()) {
			return true;
		}
		return false;
	}
	
	/**
	 * 验证密码
	 * 规则:6-18位包括一些特殊符号
	 * @param password
	 * @return
	 */
	public boolean passwordValidation(String password) {
		Matcher m = Pattern.compile("^[a-zA-Z0-9_~`.,;\':\"]{6,18}$").matcher(password);
		if(m.find()) {
			return true;
		} else {
			return false;
		}
	}
	
	public static void main(String[] args) {
		DataValidation dv = new DataValidation();
		if(dv.passwordValidation("cassss.\"ssh'.;")) {
			System.out.println("匹配成功！");
		} else {
			System.out.println("匹配失败");
		}
	}
	
}
