#server {
#    if ($host = demo.code-maven.com) {
#        return 301 https://$host$request_uri;
#    } # managed by Certbot
#
#
#  server_name demo.code-maven.com;
#  listen    [::]:80;
#  listen    80;
#
#  rewrite ^ https://demo.code-maven.com$request_uri? permanent;
#
#
#}

server {
  server_name demo.code-maven.com;
  listen    [::]:80;
  listen    80;
#  listen    [::]:443 ssl;
#  listen    443 ssl;
#    ssl_certificate /etc/letsencrypt/live/demo.code-maven.com/fullchain.pem; # managed by Certbot
#    ssl_certificate_key /etc/letsencrypt/live/demo.code-maven.com/privkey.pem; # managed by Certbot

#  location /.well-known {
#    alias /home/demo/app/letsencrypt/.well-known/;
#  }

  try_files $uri.html $uri $uri/ =404;

  location / {
      proxy_set_header Host $http_host;
      proxy_set_header X-Forwarded-Host $http_host;
      proxy_set_header X-Real-IP $remote_addr;
      proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
      proxy_pass  http://port3000;
  }


  access_log  /var/log/nginx/demo.log;
  error_log /var/log/nginx/demo.error.log;



}

upstream port3000  {
   server 127.0.0.1:3000;
}


