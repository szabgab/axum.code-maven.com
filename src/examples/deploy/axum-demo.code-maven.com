server {
  server_name axum-demo.code-maven.com;
  listen    [::]:80;
  listen    80;

  try_files $uri.html $uri $uri/ =404;

  location / {
      proxy_set_header Host $http_host;
      proxy_set_header X-Forwarded-Host $http_host;
      proxy_set_header X-Real-IP $remote_addr;
      proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
      proxy_pass  http://port3000;
  }


  access_log  /var/log/nginx/axum-demo.log;
  error_log /var/log/nginx/axum-demo.error.log;

}

upstream port3000  {
   server 127.0.0.1:3000;
}


