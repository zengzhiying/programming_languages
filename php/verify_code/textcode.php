<?php
header('Content-Type:text/html; charset=utf-8');
session_start();
$fontface='simfang.ttf';
$str='我喜欢互联网更热爱服务器端开发相信一定可以学好';
$strdb=str_split($str,3);	//注意：utf-8中文编码占3个字节
$captch_code='';
//创建底图
$image=imagecreatetruecolor(200, 60);
//定义背景颜色画笔并填充背景
$bgcolor=imagecolorallocate($image, 255, 255, 255);
imagefill($image, 0, 0, $bgcolor);
//随机汉字实现
for($i=0;$i<4;$i++){
	$fontcolor=imagecolorallocate($image, rand(0,120), rand(0,120), rand(0,120));
	$cn=$strdb[rand(0,count($strdb))];
	$captch_code.=$cn;
	imagettftext($image, mt_rand(20,24), mt_rand(-60,60), (40*$i+20), mt_rand(30,35), $fontcolor, $fontface, $cn);
}
//将验证码保存到session
$_SESSION['textcode']=$captch_code;
//增加干扰点
for($i=0;$i<200;$i++){
	$pointcolor=imagecolorallocate($image, rand(50,200), rand(50,200), rand(50,200));
	imagesetpixel($image, rand(1,199), rand(1,59), $pointcolor);
}
//增加干扰线
for($i=0;$i<3;$i++){
	$linecolor=imagecolorallocate($image, rand(80,220), rand(80,220), rand(80,220));
	imageline($image, rand(1,199), rand(1,59), rand(1,199), rand(1,59), $linecolor);
}
//告诉浏览器头传递内容
header('Content-Type:image/png');
//输出图像
imagepng($image);
//销毁图像，释放资源
imagedestroy($image);
?>