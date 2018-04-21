package net.zengzhiying.tools;

import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

/**
 * 数据加密工具类
 * @author zengzhiying
 *
 */

public class DataEncrypt {
	
	/**
	 * Sha1 单向加密算法
	 * @param inPassword
	 * @return String 40位sha码
	 */
	public String shaEncrypt(String inPassword) {
		MessageDigest sha = null;
		try {
			sha = MessageDigest.getInstance("SHA");
			byte[] byteArray = inPassword.getBytes();
			byte[] shaBytes = sha.digest(byteArray);
			StringBuffer hexValue = new StringBuffer();
			for(int i = 0;i < shaBytes.length;i++) {
				int val = ((int) shaBytes[i]) & 0xff;
				if(val < 16) {
					hexValue.append("0");
				}
				hexValue.append(Integer.toHexString(val));
			}
			return hexValue.toString();
		} catch (NoSuchAlgorithmException e) {
			e.printStackTrace();
			return null;
		}
	}
	
}
