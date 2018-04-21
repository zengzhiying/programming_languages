<?php
header('Content-Type:text/html; charset=utf-8');
session_start();
$table=array('百度','好搜','搜狗');
$table1=array('img/baidu.png', 'img/haosou.png','img/sogou.png');
$index=rand(0,2);
$value=$table[$index];
$filename=$table1[$index];
$_SESSION['imgcode']=$value;
$contents=file_get_contents($filename);
header('Content-Type:image/png');
echo $contents;
?>