package net.zengzhiying.service;

import java.util.List;

import net.zengzhiying.beans.Logs;
import net.zengzhiying.dao.BeansDao;
import net.zengzhiying.tools.DataTypeConver;

/**
 * 系统日志服务类
 * @author zengzhiying
 *
 */

public class LoggerService {
	
	/**
	 * 日志打印输入方法
	 * @return
	 */
	public List<Object> logsList() {
		BeansDao bd = new BeansDao();
		return bd.queryList("Logs", "queryLogs");
	}
	
	/**
	 * 后台添加日志
	 * @param log_type
	 * @param content
	 */
	public static void addLogs(String log_type,String content) {
		DataTypeConver dtc = new DataTypeConver();
		BeansDao bd = new BeansDao();
		Logs log = new Logs();
		log.setLog_time(dtc.newTime());
		log.setLog_type(log_type);
		log.setLog_describe(content);
		//写入日志
		bd.insertOne("Logs", "insertLogs", log);
	}
	
}
