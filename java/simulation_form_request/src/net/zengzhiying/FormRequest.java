package net.zengzhiying;

import java.io.BufferedReader;
import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.net.HttpURLConnection;
import java.net.MalformedURLException;
import java.net.URL;

/**
 * java模拟表单发送post请求实例, 包括输入框和文件上传
 * 完全模拟前端表单的请求结构向后端发送请求, 而不是服务端post或文件请求
 * @author zengzhiying
 *
 */
public class FormRequest {
    
    public static void main(String[] args) {
        String uploadFile = "test.jpg";
        String username = "小曾123a";
        String api = "http://192.168.2.137:17000/api/";
        String result = new FormRequest().sendFormFilePost(api, uploadFile, username);
        if(result != null) {
            System.out.println(result);
        } else {
            System.out.println("调用失败！");
        }
    }
    
    /**
     * 往后端或接口发送表单上传文件请求示例方法, 实际使用中根据发送内容修改
     * @param uri 接收表单请求地址
     * @param filePath 上传文件路径
     * @param username 文本字段参数
     * @return
     *        成功返回对应的响应结果
     *        调用失败返回null
     */
    public String sendFormFilePost(String uri, String filePath, String username) {
        File uploadFile = new File(filePath);
        if(!uploadFile.exists() || !uploadFile.isFile()) {
            System.out.println("文件不存在！");
            return null;
        }
        URL url;
        try {
            url = new URL(uri);
        } catch (MalformedURLException e) {
            // 创建url对象失败
            e.printStackTrace();
            return null;
        }
        try {
            HttpURLConnection connection = (HttpURLConnection) url.openConnection();
            connection.setRequestMethod("POST");
            connection.setDoInput(true);
            connection.setDoOutput(true);
//            connection.setUseCaches(false);
            connection.setRequestProperty("accept", "*/*");
            connection.setRequestProperty("Connection", "Keep-Alive");
            connection.setRequestProperty("Charset", "UTF-8");
            // 设置payload边界
            String boundary = "-------WebKitFormBoundary" + System.currentTimeMillis();
            connection.setRequestProperty("Content-Type", "multipart/form-data; boundary=" + boundary);
            // 设置文件请求正文
            StringBuilder builderFileContent = new StringBuilder();
            builderFileContent.append("--")
                .append(boundary)
                .append("\r\n")
                .append("Content-Disposition: form-data; name=\"imageByteStream\"; "
                        + "filename=\"" + uploadFile.getName() + "\"")
                .append("\r\n")
                .append("Content-Type: image/jpeg")
                .append("\r\n\r\n");
            byte[] requestPayloads = builderFileContent.toString().getBytes("utf-8");
            OutputStream output = new DataOutputStream(connection.getOutputStream());
            // 写入请求正文
            output.write(requestPayloads);
            // 将文件流推入url
            DataInputStream inputFile = new DataInputStream(new FileInputStream(uploadFile));
            int fileBytes = 0;
            byte[] bufferOutput = new byte[4096];
            while((fileBytes = inputFile.read(bufferOutput)) != -1) {
                output.write(bufferOutput, 0, fileBytes);
            }
            inputFile.close();
            // 设置参数请求正文
            StringBuilder buildNumberContent = new StringBuilder();
            buildNumberContent.append("--")
                .append(boundary)
                .append("\r\n")
                .append("Content-Disposition: form-data; name=\"user_name\"")
                .append("\r\n\r\n")
                .append(username);
                //.append("\r\n");
            output.write(buildNumberContent.toString().getBytes("utf-8"));
            // 结尾部分
            byte[] footers = ("\r\n--" + boundary + "--\r\n").getBytes("utf-8");
            output.write(footers);
            output.flush();
            output.close();
            StringBuffer buffer = new StringBuffer();
            BufferedReader reader = new BufferedReader(new InputStreamReader(connection.getInputStream()));
            String line = null;
            while((line = reader.readLine()) != null) {
                buffer.append(line);
            }
            String result = buffer.toString();
            reader.close();
            return result;
        } catch (IOException e) {
            e.printStackTrace();
            return null;
        }
    }
}
