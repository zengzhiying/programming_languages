package net.zengzhiying.service;

import java.util.List;

import net.zengzhiying.beans.Partment;
import net.zengzhiying.beans.User;
import net.zengzhiying.dao.BeansDao;
import net.zengzhiying.tools.DataTypeConver;
import net.zengzhiying.tools.DataValidation;

/**
 * 部门操作服务类
 * @author zengzhiying
 *
 */

public class PartmentService {
	
	/**
	 * 列出所有的部门
	 * @return
	 */
	public List<Object> partmentList() {
		BeansDao bd = new BeansDao();
		
		List<Object> pmList = bd.queryList("Partment", "queryAll");
		return pmList;
	}
	
	/**
	 * 插入一个部门
	 * @param pm
	 * @return
	 */
	public String addPartment(String pm_name,String pm_describe, String setup_timeStr) {
		BeansDao bd = new BeansDao();
		DataValidation dv = new DataValidation();
		Partment pm = new Partment();
		if(pm_name == null || pm_name.equals("") || setup_timeStr == null || setup_timeStr.equals("")) {
			return "empty";
		}
		
		if(pm_describe == null || pm_describe.equals("")) {
			pm_describe = "0";
		}
		//成立时间匹配
		if(dv.dateStrValidation(setup_timeStr)) {
			// 匹配成功将时间放入对象
			pm.setSetup_time(new DataTypeConver().stringToTime(setup_timeStr, "yyyy-MM-dd"));
		} else {
			return "time_error";
		}
		
		//组装剩余参数
		pm.setPm_name(pm_name);
		pm.setPm_describe(pm_describe);
		
		if(bd.insertOne("Partment", "insertPartment", pm)) {
			//System.out.println(pm.getSetup_time());
			return "success";
		} else {
			return "error";
		}
	}
	
	/**
	 * 新增部门 - 重载
	 * @param pm_name
	 * @param pm_describe
	 * @param setup_timeStr
	 * @param pm_id
	 * @return
	 */
	public String addPartment(String pm_name,String pm_describe, String setup_timeStr, String pm_id) {
		BeansDao bd = new BeansDao();
		DataValidation dv = new DataValidation();
		Partment pm = new Partment();
		if(pm_id == null || pm_id.equals("") || pm_name == null || pm_name.equals("") || setup_timeStr == null || setup_timeStr.equals("")) {
			return "empty";
		}
		
		if(pm_describe == null || pm_describe.equals("")) {
			pm_describe = "0";
		}
		//成立时间匹配
		if(dv.dateStrValidation(setup_timeStr)) {
			// 匹配成功将时间放入对象
			pm.setSetup_time(new DataTypeConver().stringToTime(setup_timeStr, "yyyy-MM-dd"));
		} else {
			return "time_error";
		}
		
		//部门id匹配
		if(!dv.posInteger(pm_id)) {
			return "id_error";
		}
		
		//组装剩余参数
		pm.setPm_name(pm_name);
		pm.setPm_describe(pm_describe);
		pm.setPm_id(Integer.valueOf(pm_id));
		
		if(bd.updateOne("Partment", "updatePartment", pm)) {
			//System.out.println(pm.getSetup_time());
			return "success";
		} else {
			return "error";
		}
	}
	
	/**
	 * 删除一个部门
	 * @param pm_id
	 * @return
	 */
	public String deletePartment(String pm_id) {
		DataValidation dv = new DataValidation();
		BeansDao bd = new BeansDao();
		// pm_id验证
		if(pm_id == null || pm_id.equals("")) {
			return "error";
		}
		if(!dv.posInteger(pm_id)) {
			return "error";
		}
		int pm_id_1 = Integer.valueOf(pm_id);
		User u = (User) bd.queryOne("User", "PartmentToUser", pm_id_1);
		
		if(u != null) {
			// 该部门下有员工，无法删除
			return "filed";
		}
		if(bd.deleteOne("Partment", "deletePartment", Integer.valueOf(pm_id))) {
			//删除成功
			return "success";
		} else {
			//删除失败可能是范围超过限制或者数据库问题
			return "filed_error";
		}
	}
	
	/**
	 * 根据部门编号查询部门详情
	 * @param pm_id
	 * @return
	 */
	public Partment editPartment(String pm_id) {
		DataValidation dv = new DataValidation();
		BeansDao bd = new BeansDao();
		// pm_id验证
		if(pm_id == null || pm_id.equals("")) {
			return null;
		}
		if(!dv.posInteger(pm_id)) {
			return null;
		}
		return (Partment) bd.queryOne("Partment", "queryName", Integer.valueOf(pm_id));
	}
	
	/**
	 * 统计某个部门中除部门经理外员工的总数
	 * @param partment_id
	 * @return
	 */
	public int staffCount(int partment_id) {
		BeansDao bd = new BeansDao();
		List<Object> numList = bd.queryList("User", "PartmentToUser1", partment_id);
		return numList.size();
	}
	
	public static void main(String[] args) {
		PartmentService ps = new PartmentService();
		//ps.partmentList();
		Partment pm = new Partment();
		pm.setPm_name(null);
		pm.toString();
	}
	
}
