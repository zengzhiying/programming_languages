<?php
require_once 'dbcon/conn.php';
$one_sql="SELECT area_id,area_name from tb_area where area_parent=0";
if($result=mysql_query($one_sql,$conn)){
	while($row=mysql_fetch_assoc($result)){
		$one_data[]=$row;
	}
}else{
	echo '查询失败！';
}
$two_sql="SELECT area_id,area_name from tb_area where area_parent=".$one_data[0]['area_id'];
if($result=mysql_query($two_sql,$conn)){
	while ($row=mysql_fetch_assoc($result)) {
		$two_data[]=$row;
	}
}else{
	echo "查询失败！";
}
?>
<!DOCTYPE html>
<html>
<head>
	<meta charset="utf-8">
	<title>Ajax三级联动</title>
	<link rel="stylesheet" type="text/css" href="css/zui.css">
	<style type="text/css">
	div{
		margin:auto;
		width: 980px;
		margin-top: 123px;
	}
	.select_style{
		width:234px;
	}
	</style>
</head>
<body>
	<div>
		<form method="get">
			<label for="one_area">请选择一级地区：</label>
			<select name="one_area" id="one_area" class="form-control select_style" onchange="Linkage(this.value,'1')">
				<?php
				foreach ($one_data as $key => $value) {
				?>
				<option value="<?php echo $value['area_id']; ?>"><?php echo $value['area_name']; ?></option>
				<?php
				}
				?>
			</select>
			<br />
			<label for="two_area">请选择二级地区：</label>
			<select name="two_area" id="two_area" class="form-control select_style" onchange="Linkage(this.value,'2')">
				<?php
				foreach ($two_data as $key => $value) {
				?>
				<option value="<?php echo $value['area_id']; ?>"><?php echo $value['area_name']; ?></option>
				<?php
				}
				?>
			</select>
			<br />
			<label for="three_area">请选择三级地区：</label>
			<select name="three_area" id="three_area" class="form-control select_style">
				<option>请选择</option>
			</select>
		</form>
	</div>
	<script type="text/javascript" src="jquery/jquery-1.11.1.min.js"></script>
	<script type="text/javascript">


	$(document).ready(function() {
		//这个方法内只能用jQuery自己的事件绑定，不能写其他方法
		var area_id_2 = "<?php echo $two_data[0]['area_id'] ?>";
		//三级菜单初始化
		$.getJSON("server.php",{'area_id':area_id_2, 'r':'1'},function(data){
			$("#three_area").html("");
			for(var i=0;i<getJsonObjLength(data);i++){
				$("#three_area").append("<option value='"+data[i]['area_id']+"'>"+data[i]['area_name']+"</option>");
			}
		});
	});

	function Linkage(area,r){
		$.getJSON("server.php",{'area_id':area},function(data){
			if(r=='1'){
				//二级菜单变化
				$("#two_area").html("");
				for(var i=0;i<getJsonObjLength(data);i++){
					$("#two_area").append("<option value='"+data[i]['area_id']+"'>"+data[i]['area_name']+"</option>");
				}
				//三级菜单同时变化
				$("#three_area").html("");
				$.get("server.php",{'area_id': $("#two_area").val()},function(data) {
					for (var i = 0; i <= getJsonObjLength(data); i++) {
						$("#three_area").append("<option value='"+data[i]['area_id']+"'>"+data[i]['area_name']+"</option>");
					};
				},"json");
			}else if(r=='2'){
				$("#three_area").html("");
				for(var i=0;i<getJsonObjLength(data);i++){
					$("#three_area").append("<option value='"+data[i]['area_id']+"'>"+data[i]['area_name']+"</option>");
				}
			}
		});
	}

	//统计json返回的键值对个数
	function getJsonObjLength(jsonObj){
		var Length=0;
		for (var item in jsonObj){
			Length++;
		}
		return Length;
	}
	</script>
</body>
</html>