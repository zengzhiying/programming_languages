<?php
header('Content-Type:text/html; charset=utf-8');
session_start();
$captch_code='';
//创建底图
$image=imagecreatetruecolor(100, 30);
//定义背景颜色画笔并填充背景
$bgcolor=imagecolorallocate($image, 255, 255, 255);
imagefill($image, 0, 0, $bgcolor);
//随机数字和颜色实现
/*
for($i=0;$i<4;$i++){
	$fontsize=6;
	$fontcolor=imagecolorallocate($image, rand(0,120), rand(0,120), rand(0,130));
	$fontcontent=rand(0,9);
	$x=($i*100/4)+rand(5,10);
	$y=rand(5,10);
	imagestring($image, $fontsize, $x, $y, $fontcontent, $fontcolor);
}
*/
//随机字母加数字实现
for($i=0;$i<4;$i++){
	$fontsize=6;
	$fontcolor=imagecolorallocate($image, rand(0,120), rand(0,120), rand(0,120));
	$data='qwertyuiopasdfghjklzxcvbnm1234567890';	//字典字符串
	$fontcontent=substr($data, rand(0,strlen($data)-1),1);	//随机截取一个字符
	$captch_code.=$fontcontent;	//变量获取验证码
	$x=($i*100/4)+rand(5,10);
	$y=rand(5,10);
	imagestring($image, $fontsize, $x, $y, $fontcontent, $fontcolor);
}
//将验证码保存到session
$_SESSION['authcode']=$captch_code;
//增加干扰点
for($i=0;$i<200;$i++){
	$pointcolor=imagecolorallocate($image, rand(50,200), rand(50,200), rand(50,200));
	imagesetpixel($image, rand(1,99), rand(1,29), $pointcolor);
}
//增加干扰线
for($i=0;$i<3;$i++){
	$linecolor=imagecolorallocate($image, rand(80,220), rand(80,220), rand(80,220));
	imageline($image, rand(1,99), rand(1,29), rand(1,99), rand(1,29), $linecolor);
}
//告诉浏览器头传递内容
header('Content-Type:image/png');
//输出图像
imagepng($image);
//销毁图像，释放资源
imagedestroy($image);
?>