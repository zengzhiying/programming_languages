package net.zengzhiying.tests;

import java.util.ArrayList;
import java.util.List;

import net.zengzhiying.beans.Message;
import net.zengzhiying.dao.MessageDao;

public class MyBatisTest {

	public static void main(String[] args) {
//		Message msg = MessageDao.queryOne("querymessage");
//		System.out.println(msg.getName());
		List<Message> msgList = new ArrayList<Message>();
		//msgList = MessageDao.queryList("querymessage");
//		for(Message ml:msgList) {
//			System.out.println(ml.getName());
//		}
		
		//msgList.clear();
//		msgList = MessageDao.queryList("queryParam", "tts");
//		System.out.println(msgList.size());
//		for(Message ml:msgList) {
//			System.out.println(ml.getName());
//		}
		
		msgList.clear();
		Message msg = new Message();
		msg.setName("zzy");
		msg.setPassword("t");
		
		MessageDao md = new MessageDao();
		msgList = md.queryList("queryMessage",msg);
		for(Message ml:msgList) {
			System.out.println(ml.getPassword());
		}
		
		//删除一条
		//md.deleteOne("Message.deleteOne", 5);
		
		//更新一条
		//md.updateOne("Message.updateOne", new Message(3,"hdd","hdd"));
		
//		List<Integer> intList = new ArrayList<Integer>();
//		intList.add(6);
//		intList.add(7);
		//删除多条
		//md.deleteBatch("Message.deleteBatch", intList);
		//添加一条
//		Message m = new Message();
//		m.setName("skksk");
//		m.setPassword("xingkong");
//		md.insertOne("Message.insertOne", m);
		
		//批量添加100条
		//时间统计
		long start = System.currentTimeMillis();
		msgList.clear();
		for(int i = 0;i <= 100;i++) {
			Message m = new Message();
			m.setName("Name Data");
			m.setPassword("Pass Data");
			//md.insertOne("Message.insertOne", m);
			msgList.add(m);
		}
		md.insertBatch("Message.insertOne", msgList);
		long end = System.currentTimeMillis();
		System.out.println("写入数据用时：" + (end-start)/1000 + "s");
		
	}

}
