package net.zengzhiying.tests;

import net.zengzhiying.tools.DataEncrypt;

public class EncryptTest {
	
	public static void main(String[] args) {
		DataEncrypt de = new DataEncrypt();
		System.out.println(de.shaEncrypt("admin"));
	}
	
}
