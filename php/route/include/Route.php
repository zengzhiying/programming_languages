<?php
/*
  $_SERVER['PHP_SELF']; 相对于根目录的实际路径，包括所有隐藏的，不包括参数 
  $_SERVER['DOCUMENT_ROOT'];  服务器根目录系统的目录
  $_SERVER['REQUEST_URI'];	url显示的目录，包括参数
  $_SERVER["QUERY_STRING"]; 获取url参数，仅获取get参数
 */


$location = stripos($_SERVER['PHP_SELF'], 'index.php');
$run = substr($_SERVER['PHP_SELF'], 0,$location);	//获取当前运行目录

$url_lang = strlen($_SERVER['REQUEST_URI']);
$get_route=substr($_SERVER['REQUEST_URI'], $location,$url_lang);	//获取当前目录下全部地址

//获取纯路由字符串
if($parame=stripos($get_route, '?')){
	$route_str = substr($get_route, 0,$parame);
}else{
	$route_str = $get_route;
}

$route_arr=explode('/', $route_str);	//分割得到路由数组
//print_r($route_arr);
//$route_arr无论什么情况0都存在，1需要再次判断存在性



//默认情况
if($route_arr[0]==''||($route_arr[0]=='index.php'&&!isset($route_arr[1]))||($route_arr[0]=='index.php'&&isset($route_arr[1])&&$route_arr[1]=='')){
	if(!file_exists(Apps.'Index.class.php')){
		echo '文件不存在！';
	}else{
		require_once Apps.'Index.class.php';
		if(!class_exists('Index')){
			echo '模块未定义！';
		}else{
			$obj = new Index();
			if(!method_exists('Index', 'index')){
				echo '方法不存在！';
			}else{
				//不用执行，默认执行
			}
		}
	}
}


//访问带index.php入口的路由情况
if($route_arr[0]=='index.php'&&isset($route_arr[1])&&$route_arr[1]!=''){
	$route_arr[1]=ucfirst($route_arr[1]);	//避免大小写干扰
	if(!file_exists(Apps.$route_arr[1].'.class.php')){
		echo '文件不存在！';
	}else{
		require_once Apps.$route_arr[1].'.class.php';
		if(!class_exists($route_arr[1])){
			echo '模块未定义！';
		}else{
			$obj = new $route_arr[1]();
			if(!isset($route_arr[2])||(isset($route_arr[2])&&$route_arr[2]=='')){
				if(!method_exists($route_arr[1], 'index')){
					echo '方法不存在！';
				}else{
					//默认执行
					$obj->index();
				}
			}
			if(isset($route_arr[2])&&$route_arr[2]!=''){
				if(!method_exists($route_arr[1], $route_arr[2])){
					echo '方法不存在！';
				}else{
					$obj->$route_arr[2]();
				}
			}
		}
	}
}


//访问URL重写过的路由
if($route_arr[0]!=''&&$route_arr[0]!='index.php'){
	$route_arr[0]=ucfirst($route_arr[0]);	//排除大小写干扰
	if(!file_exists(Apps.$route_arr[0].'.class.php')){
		echo '文件不存在！';
	}else{
		require_once Apps.$route_arr[0].'.class.php';
		if(!class_exists($route_arr[0])){
			echo '模块未定义！';
		}else{
			$obj = new $route_arr[0]();
			if(!isset($route_arr[1])||(isset($route_arr[1])&&$route_arr[1]=='')){
				if(!method_exists($route_arr[0], 'index')){
					echo '方法不存在！';
				}else{
					//默认执行
					$obj->index();
				}
			}
			if(isset($route_arr[1])&&$route_arr[1]!=''){
				if(!method_exists($route_arr[0], $route_arr[1])){
					echo '方法不存在！';
				}else{
					$obj->$route_arr[1]();
				}
			}
		}
	}
}
?>