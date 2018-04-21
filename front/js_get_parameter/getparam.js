var jsFileName="getparam.js";
var rName = new RegExp(jsFileName+"(\\?(.*))?$");
var jss=document.getElementsByTagName("script");
var params = new Array();   //数组名绝不可以用name
var k=0;
for (var i = 0;i < jss.length; i++){
    var j = jss[i];
    if (j.src&&j.src.match(rName)){
        var oo = j.src.match(rName)[2];
        if (oo&&(t = oo.match(/([^&=]+)=([^=&]+)/g))){
            for (var l = 0; l < t.length; l++){
                r = t[l];
                var tt = r.match(/([^&=]+)=([^=&]+)/);
                if (tt){
                    //多个参数值输出
                    //document.write('参数：' + tt[1] + '，参数值：' + tt[2] + '<br />');
                    params[k]=tt[2];
                    //document.write(params[k]+'<br />');
                    k++;
                    
                }
            }
        }
    }
}
//输出参数值
document.write("参数1: " + params[0] + "<br />")
document.write("参数2: " + params[1])
