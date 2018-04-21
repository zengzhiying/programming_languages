package net.zengzhiying.tools;

import java.text.ParseException;
import java.text.SimpleDateFormat;
import java.util.Date;

/**
 * 数据类型转换类
 * @author zengzhiying
 *
 */

public class DataTypeConver {
	
	/**
	 * 时间戳转日期字符串
	 * @param time
	 * @return
	 */
	public String timeToString(long time, String dateFormat) {
		// String dateFormat = "yyyy-MM-dd HH:mm:ss";
        SimpleDateFormat sdf = new SimpleDateFormat(dateFormat);
        return sdf.format(new Date(time * 1000));
        
	}
	
	/**
	 * 日期字符串转时间戳
	 * @param timeStr
	 * @return
	 */
	public long stringToTime(String timeStr, String dateFormat) {
		//String dateFormat = "yyyy-MM-dd HH:mm:ss";
		SimpleDateFormat sdf = new SimpleDateFormat(dateFormat);
		try {
			return (long) (sdf.parse(timeStr).getTime()/1000);
		} catch (ParseException e) {
			e.printStackTrace();
			return 0;
		}
	}
	
	/**
	 * 获取当前时间戳
	 * @return
	 */
	public long newTime() {
		long time = System.currentTimeMillis()/1000;
		return time;
	}
	
	public static void main(String[] args) {
		DataTypeConver dtc = new DataTypeConver();
		System.out.println(dtc.newTime());
		System.out.println(dtc.timeToString(dtc.newTime(), "yyyy-MM-dd HH:mm:ss"));
		System.out.println(dtc.stringToTime("2016-05-29","yyyy-MM-dd"));
	}
	
}
