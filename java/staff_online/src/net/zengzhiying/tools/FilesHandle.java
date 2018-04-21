package net.zengzhiying.tools;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.RandomAccessFile;

/**
 * 文件操作类
 * @author zengzhiying
 *
 */

public class FilesHandle {
	
	/**
	 * 向文件写入内容 ，从前面开始写不影响后面的内容
	 * 如果文件不存在，会尝试重新建立
	 * @param uri
	 * @param text
	 * @return
	 */
	public int fileWrite(String uri, String text) {
		final int PASS = 0;	//成功
		final int ERROR_NOTFOUND = 1;	//文件不存在或错误
		final int ERROR_IO = 2;	//文件写入出错
		File f = new File(uri);
		try {
			RandomAccessFile raf = new RandomAccessFile(f, "rw");
			//拆分为字节数组
			byte[] bytes = text.getBytes();
			//写入字节数组
			raf.write(bytes);
			//关闭资源
			raf.close();
			return PASS;
		} catch (FileNotFoundException e) {
			e.printStackTrace();
			return ERROR_NOTFOUND;
		} catch (IOException e) {
			e.printStackTrace();
			return ERROR_IO;
		}
	}
	
	
	/**
	 * 文件内容读取
	 * 将内容返回一个字符串
	 * @param uri
	 * @return
	 */
	public String fileRead(String uri) {
		File f = new File(uri);
		try {
			RandomAccessFile raf = new RandomAccessFile(f, "r");
			//移动指针到头部
			raf.seek(0);
			//定义字节数组并读入
			byte[] buf = new byte[(int) raf.length()];
			raf.read(buf);
			raf.close();
			return (new String(buf));
		} catch (FileNotFoundException e) {
			e.printStackTrace();
			return null;
		} catch (IOException e) {
			e.printStackTrace();
			return null;
		}
	}
	
}
