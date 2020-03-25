<?php
header("Content-Type:text/html; charset=utf-8");
function export_excel($data = array(), $title = array(), $filename = 'report') {
    // 设置header头
    header("Content-type:application/octet-stream");
    header("Accept-Ranges:bytes");
    header("Content-type:application/vnd.ms-excel");  
    header("Content-Disposition:attachment;filename=" . $filename . ".xls");
    header("Pragma: no-cache");
    header("Expires: 0");
    // 导出标题
    if (!empty($title)){
        foreach ($title as $k => $v) {
            $title[$k]=iconv("UTF-8", "GB2312",$v);
        }
        $title= implode("\t", $title);
        echo "$title\n";
    }
    // 导出数据
    if (!empty($data)){
        foreach($data as $key=>$val){
            foreach ($val as $ck => $cv) {
                $data[$key][$ck]=iconv("UTF-8", "GB2312", $cv);
            }
            $data[$key]=implode("\t", $data[$key]);
            
        }
        echo implode("\n",$data);
    }
}

// 开始导出
$data = array(
    0 => ["2014-03-27 08:00:30",68.23, '20140327000001'],
    1 => ["2014-03-27 08:30:00", 76.0, '20140327000002'] 
    );
$title = array(
    "时间","账单金额","编号"
    );
export_excel($data, $title, 'test');

