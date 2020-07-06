package com.monchickey;

import com.alibaba.fastjson.JSON;
import com.alibaba.fastjson.JSONObject;
import org.apache.http.HttpEntity;
import org.apache.http.NameValuePair;
import org.apache.http.StatusLine;
import org.apache.http.client.entity.UrlEncodedFormEntity;
import org.apache.http.client.methods.CloseableHttpResponse;
import org.apache.http.client.methods.HttpGet;
import org.apache.http.client.methods.HttpPost;
import org.apache.http.entity.ContentType;
import org.apache.http.entity.mime.MultipartEntityBuilder;
import org.apache.http.impl.client.CloseableHttpClient;
import org.apache.http.impl.client.HttpClients;
import org.apache.http.util.EntityUtils;

import java.io.*;
import java.util.ArrayList;
import java.util.List;

public class HttpClientExample {

    public static byte[] readFileAsBytes(String filename) throws IOException {
        File f = new File(filename);
        long fileSize = f.length();
        ByteArrayOutputStream os = new ByteArrayOutputStream((int) f.length());
        BufferedInputStream inputStream = new BufferedInputStream(new FileInputStream(filename));
        int size;
        byte[] buf = new byte[4096];
        while ((size = inputStream.read(buf)) != -1) {
            os.write(buf, 0, size);
        }
        os.flush();
        byte[] fileBytes = os.toByteArray();
        inputStream.close();
        os.close();
        return fileBytes;
    }

    public static void main(String[] args) throws UnsupportedEncodingException {

        byte[] fileBytes;
        try {
            fileBytes = readFileAsBytes("./example.jpg");
        } catch (IOException e) {
            e.printStackTrace();
            return;
        }

        CloseableHttpClient httpClient = HttpClients.createDefault();
        HttpGet httpGet = new HttpGet("http://192.168.0.31:9333/dir/assign");
        String fid = null;
        String url = null;
        String publicUrl = null;
        try {
            CloseableHttpResponse response = httpClient.execute(httpGet);
            StatusLine statusLine = response.getStatusLine();
            System.out.println(statusLine);
            if(statusLine.getStatusCode() >= 200 && statusLine.getStatusCode() < 300) {
                HttpEntity entity = response.getEntity();
                String respText = EntityUtils.toString(entity, "utf-8");
                JSONObject respJson = JSON.parseObject(respText);
                if(respJson.containsKey("fid"))
                    fid = respJson.getString("fid");

                if(respJson.containsKey("url"))
                    url = respJson.getString("url");

                if(respJson.containsKey("publicUrl"))
                    publicUrl = respJson.getString("publicUrl");
                EntityUtils.consume(entity);
            } else {
                System.out.println("status code error: " + statusLine.getStatusCode());
            }
            response.close();
        } catch (IOException e) {
            e.printStackTrace();
        }

        System.out.println("fid: " + fid + " url: " + url);
        if(null == fid || null == url || null == publicUrl) {
            try {
                httpClient.close();
            } catch (IOException e) {
                e.printStackTrace();
            }
            return;
        }

        if(publicUrl.charAt(publicUrl.length() - 1) == '/')
            publicUrl = publicUrl.substring(0, publicUrl.length() - 1);

        HttpPost httpPost = new HttpPost("http://" + url + "/" + fid);
        List<NameValuePair> nvps = new ArrayList<>();
        httpPost.setEntity(new UrlEncodedFormEntity(nvps));
        MultipartEntityBuilder builder = MultipartEntityBuilder.create();
        builder.addBinaryBody("file", fileBytes, ContentType.IMAGE_JPEG, fid + ".jpg");
        httpPost.setEntity(builder.build());
        try {
            CloseableHttpResponse httpResponse = httpClient.execute(httpPost);
            StatusLine statusLine = httpResponse.getStatusLine();
            if(statusLine.getStatusCode() >= 200 && statusLine.getStatusCode() < 300) {
                HttpEntity entity = httpResponse.getEntity();
                System.out.println(EntityUtils.toString(entity, "utf-8"));
                String weedUrl = publicUrl + "/" + fid + ".jpg";
                System.out.println("weed url: " + weedUrl);
                EntityUtils.consume(entity);
            } else {
                System.out.println("status code error: " + statusLine.getStatusCode());
            }
            httpResponse.close();
        } catch (IOException e) {
            e.printStackTrace();
        }

        try {
            httpClient.close();
        } catch (IOException e) {
            e.printStackTrace();
        }

    }
}
