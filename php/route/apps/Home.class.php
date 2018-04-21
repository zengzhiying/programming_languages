<?php
class Home{
	/**
	 * 此类只做演示使用，请新建其他类
	 */
	public function index(){
		echo "Home类 index方法<br />";
		if(!empty($_GET['r'])){
			echo $_GET['r'];
		}

		if(isset($_POST['name1'])){
			echo $_POST['name1'];
		}
	}
	public function index1(){
		echo "Home类 index1方法";
	}
}
?>