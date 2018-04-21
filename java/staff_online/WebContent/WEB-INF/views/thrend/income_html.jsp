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
		            text: '企业收入状况统计',
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
		                text: 'y轴数据 (万元)'
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
		            valueSuffix: '万元'
		        },
		        legend: {
		            layout: 'vertical',
		            align: 'right',
		            verticalAlign: 'middle',
		            borderWidth: 0
		        },
		        series: [{
		            name: '收入',
		            data: [89.7, 76.0, 91.2, 68.3, 56.1, 75.3, 65.2, 68.5, 59.3, 78.3, 81.9]
		        }, {
		            name: '投资',
		            data: [36.3, 47.2, 56.1, 23.5, 58.0, 38.4, 67.3, 60.1, 36.1, 45.1, 50.6]
		        }]
		    });
		});
				
	</script>
</head>
<body>
	<div id="container" style=""></div>
	
	
	
	
	
	<script type="text/javascript">
	
	$(function () {
	    // Set up the chart
	    var chart = new Highcharts.Chart({
	        chart: {
	            renderTo: 'container1',
	            type: 'column',
	            margin: 75,
	            options3d: {
	                enabled: true,
	                alpha: 15,
	                beta: 15,
	                depth: 50,
	                viewDistance: 25
	            }
	        },
	        title: {
	            text: '相对应的企业收入柱状图统计'
	        },
	        subtitle: {
	            text: ''
	        },
	        yAxis: {
	            title: {
	                text: ' (万元)'
	            }
	        },
	        plotOptions: {
	            column: {
	                depth: 25
	            }
	        },
	        tooltip: {
	            valueSuffix: '万元'
	        },
	        series: [{
	        	name:"收入",
	            data: [89.7, 76.0, 91.2, 68.3, 56.1, 75.3, 65.2, 68.5, 59.3, 78.3, 81.9]
	        },{
	        	name:"支出",
	            data: [36.3, 47.2, 56.1, 23.5, 58.0, 38.4, 67.3, 60.1, 36.1, 45.1, 50.6]
	        }]
	    });
	    

	   

	    function showValues() {
	        $('#R0-value').html(chart.options.chart.options3d.alpha);
	        $('#R1-value').html(chart.options.chart.options3d.beta);
	    }
	    showValues();
	});
	</script>
	<br />
	<br />
	<br />
	<br />
	<div id="container1" style="min-width:700px;height:400px"></div>
</body>
</html>
    