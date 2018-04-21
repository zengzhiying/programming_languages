<?php
class Index{
	/**
	 * 路由默认类
	 * 此类无任何实际作用，只是体现默认用法请在其他类中编写算法
	 * 类中的方法不要和类重名，否则会初始化执行
	 */
	public function index(){
		//默认方法
		//php5.3以上默认实例化对象时会被调用
		//echo "方法 index 被调用";
		echo '欢迎使用路由分发系统';
	}
	public function index1(){
		echo "方法 index1 被调用";
	}
}
?>