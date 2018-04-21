<?php
require_once 'dbcon/conn.php';
//服务端联动请求返回下级数据
if(!empty($_GET['area_id'])){
	//查询操作
	$area_id = $_GET['area_id'];
	$sql = "SELECT * FROM tb_area where area_parent=".$area_id;
	if($result = mysql_query($sql, $conn)){
		while ($row = mysql_fetch_assoc($result)) {
			$data[]=$row;
		}
		echo json_encode($data);
	}else{
		echo '{"":""}';
	}
}
?>