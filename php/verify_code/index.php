<!DOCTYPE html>
<html>
<head>
	<meta charset="utf-8">
	<title>简单验证码实现</title>
	<style type="text/css">
	div{
		width: 980px;
		margin:auto;
		position: absolute;
		left:130px;
		top:200px;
		text-align: center;
	}
	a{
		text-decoration: none;
	}
	</style>
</head>
<body>
	<div>
		<form action="./" method="post">
			<p>
				验证码图片：<img src="./captcha.php?r=<?php echo rand(); ?>" id="captcha_img1" onclick="captch1()" />
				<a href="javascript:void(0)" onclick="captch1()">看不清？换一个</a>
				<script type="text/javascript">
				function captch1(){
					document.getElementById("captcha_img1").src="./captcha.php?r="+Math.random();
				}
				</script>
			</p>
			<p>请输入验证码内容：<input type="text" name="authcode" /></p>
			<p><input type="submit" value="提交" style="padding:6px 20px;" /></p>
		</form>
		<br />
		<form action="./" method="post">
			<p>
				验证码图片：<img src="./imgcode.php?r=<?php echo rand(); ?>" id="captcha_img2" onclick="captch2()" width="200px" height="76px" />
				<a href="javascript:void(0)" onclick="captch2()">看不清？换一个</a>
				<script type="text/javascript">
				function captch2(){
					document.getElementById("captcha_img2").src="./imgcode.php?r="+Math.random();
				}
				</script>
			</p>
			<p>请输入搜索引擎名称：<input type="text" name="imgcode" /></p>
			<p><input type="submit" value="提交" style="padding:6px 20px;" /></p>
		</form>
		<br />
		<form action="./" method="post">
			<p>
				验证码图片：<img src="./textcode.php?r=<?php echo rand(); ?>" id="captcha_img3" onclick="captch3()" />
				<a href="javascript:void(0)" onclick="captch3()">看不清？换一个</a>
				<script type="text/javascript">
				function captch3(){
					document.getElementById("captcha_img3").src="./textcode.php?r="+Math.random();
				}
				</script>
			</p>
			<p>请输入验证码内容：<input type="text" name="textcode" /></p>
			<p><input type="submit" value="提交" style="padding:6px 20px;" /></p>
		</form>
	</div>
</body>
</html>
<?php
if(isset($_POST['authcode'])){
	session_start();
	//将用户输入大小写转换并判断
	if(strtolower($_POST['authcode'])==$_SESSION['authcode']){
		echo "输入正确！";
	}else{
		echo "输入错误！";
	}
	exit();
}
if(isset($_POST['imgcode'])){
	session_start();
	//将用户输入大小写转换并判断
	if($_POST['imgcode']==$_SESSION['imgcode']){
		echo "输入正确！";
	}else{
		echo "输入错误！";
	}
	exit();
}
if(isset($_POST['textcode'])){
	session_start();
	//将用户输入大小写转换并判断
	if($_POST['textcode']==$_SESSION['textcode']){
		echo "输入正确！";
	}else{
		echo "输入错误！";
	}
	exit();
}
?>