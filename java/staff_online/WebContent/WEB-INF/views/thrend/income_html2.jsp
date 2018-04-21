<%@ page language="java" contentType="text/html; charset=UTF-8"
    pageEncoding="UTF-8"%>

<!doctype html>
<html lang="en">
<head>
	<meta charset="utf-8">
	<script type="text/javascript" src="http://cdn.hcharts.cn/jquery/jquery-1.8.3.min.js"></script>
	<script type="text/javascript" src="http://cdn.hcharts.cn/highcharts/highcharts.js"></script>
	<script type="text/javascript" src="http://cdn.hcharts.cn/highcharts/exporting.js"></script>
	<script src="http://cdn.hcharts.cn/highcharts/modules/exporting.js" type="text/javascript"></script> 
	<script>
		$(function () {
		    $('#container').highcharts({
		        title: {
		            text: '企业产出状况统计',
		            x: -20 //center
		        },
		        subtitle: {
		            text: '',
		            x: -20
		        },
		        xAxis: {
		            categories: ['2015.08', '2015.09', '2015.10', '2015.11', '2015.12', '2016.01','2016.02', '2016.03', '2016.04', '2016.05', '2016.06'],
		            tickmarkPlacement:'on'
		        },
		        yAxis: {
		            title: {
		                text: 'y轴数据 (数量单位)'
		            },
		            plotLines: [{
		                value: 0,
		                width: 1,
		                color: '#808080'
		            }],
		            labels:{
		            	//step:2
		            }
		        },
		        tooltip: {
		            valueSuffix: '件'
		        },
		        legend: {
		            layout: 'vertical',
		            align: 'right',
		            verticalAlign: 'middle',
		            borderWidth: 0
		        },
		        series: [{
		            name: '产品1',
		            data: [8923, 8912, 7805, 6803, 6823, 7623, 8912, 6815, 5983, 7893, 8169]
		        }, {
		            name: '产品2',
		            data: [1256, 2365, 1890, 899, 1256, 2933, 1929, 1282, 1034, 1938, 1383]
		        },{
		            name: '产品3',
		            data: [2837, 2038, 1928, 1828, 1932, 1918, 2819, 1836, 1263, 1283, 1827]
		        },{
		            name: '产品4',
		            data: [5837, 4827, 5938, 5261, 6283, 6129, 5928, 4948, 5837, 6281, 6371]
		        }]
		    });
		});
				
	</script>
</head>
<body>
	<div id="container" style=""></div>
	
	
	
	
	
	<script type="text/javascript">
	
	$(function () {
	    $('#container1').highcharts({
	        chart: {
	            type: 'pie',
	            options3d: {
	                enabled: true,
	                alpha: 45,
	                beta: 0
	            }
	        },
	        title: {
	            text: '企业产品市场份额统计'
	        },
	        tooltip: {
	            pointFormat: '{series.name}: <b>{point.percentage:.1f}%</b>'
	        },
	        plotOptions: {
	            pie: {
	                allowPointSelect: true,
	                cursor: 'pointer',
	                depth: 35,
	                dataLabels: {
	                    enabled: true,
	                    format: '{point.name}'
	                }
	            }
	        },
	        series: [{
	            type: 'pie',
	            name: '产品占比：',
	            data: [
	                ['产品1',   55.0],
	                ['产品2',   12.3],
	                {
	                    name: '产品3',
	                    y: 8.8,
	                    sliced: true,
	                    selected: true
	                },
	                ['产品4',    18.3],
	                ['产品5',     3.3],
	                ['其他',   2.3]
	            ]
	        }]
	    });
	});	
	</script>
	<br />
	<br />
	<br />
	<br />
	<div id="container1" style="min-width:700px;height:400px"></div>
</body>
</html>
    