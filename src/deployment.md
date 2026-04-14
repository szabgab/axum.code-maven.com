# Deployment

* Create a Linode:
    * Region: does not matter
    * Linux Distribution: Ubuntu 24.04 LTS
    * Shared CPU: Nanode 1GB ($5 / month)
    * Linode Label: axum-demo
    * Root Password: Pick something good!
    * SSH Keys:  I selectec the one I already have on file.
    * Public Interface Firewall: No firewall

Click in "Create Linode"

It will take  about a minute to create the Linode and you'll see your IP there.

* ssh root@IP

```
# apt update
# apt upgrade -y
# apt install -y nginx
```

* Create user

```
# adduser demo --gecos ''
# cp -r /root/.ssh/ /home/demo/
# chown -R demo:demo /home/demo/.ssh/

# reboot
```

ssh demo@IP


As user `demo`:

```
$ mkdir app
```

## Release the application

e.g. go to the echo-get example and run

```
cargo build --release
```

This will create a binary in the `target/release` folder. In the case of the echo-get example this binary is called `echo-get`.

```
docker run --rm -it -v "$PWD:/src" --user ubuntu -w /src szabgab/rust:latest /home/ubuntu/.cargo/bin/cargo build --release
```


Upload to the server

scp target/release/echo-get demo@IP:app/axum-demo



## Verify that the application runs


ssh demo@IP

```
cd app
./demo
```

This should run the server

In another terminal `ssh demo@IP` again and try:

```
curl http://localhost:3000
```

That should show the main page

## Set it up as a service

{% embed include file="src/examples/deploy/axum-demo.service" %}

* Upload the `demo.service` file to `/etc/systemd/system/demo.service`
* scp demo.service root@IP:/etc/systemd/system/demo.service

ssh root@IP

```
# systemctl daemon-reload
# systemctl enable demo.service
# systemctl start demo.service
```

## Verify that the application runs as a service:

Running `curl http://localhost:3000` again (on the server) should return the page again.


## nginx

scp axum-demo.code-maven.com     root@IP:/etc/nginx/sites-available/

```
# cd /etc/nginx/sites-enabled
# ln -s /etc/nginx/sites-available/axum-demo.code-maven.com
# systemctl restart nginx
```


{% embed include file="src/examples/deploy/axum-demo.code-maven.com" %}


## HTTPS Certificate

* Install certbot for Let's Encode certificate following https://certbot.eff.org/instructions?ws=nginx&os=snap :

```
# snap install --classic certbot
# ln -s /snap/bin/certbot /usr/bin/certbot
# certbot --nginx
```

Domain name to certify: axum-demo.code-maven.com


