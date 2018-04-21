<?php
header('Content-type:text/html; charset=utf-8');
$host='localhost';	//数据库连接地址
$user='root';	//数据库用户名
$password='xingkong';	//数据库密码
$db_name='devproject';	//数据库名字

$conn=mysql_connect($host,$user,$password) or die ("连接数据库服务器失败！".mysql_error());
if(mysql_select_db($db_name)){
	//echo '数据库选择成功';
}else{
	die("数据库选择失败！");
}
//设置数据字符集
mysql_query("set names 'utf8'",$conn);
?>