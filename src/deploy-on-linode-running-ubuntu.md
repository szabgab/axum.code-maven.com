# Deploy on Linode running Ubuntu

In the following instructions I used the name `demo`, `axum-demo`, and axum-demo.code-maven.com.
You'll probably have to replace those values.

* Create a [Linode](https://linode.com/):
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

Because for me both my desktop and the server runs Ubuntu, this makes sense, but if you have different Operating Systems or different distributions then you might need to either setup cross-compilation, or use a Docker container to build the executable.

An alternatively could be this command:

```
docker run --rm -it -v "$PWD:/src" --user ubuntu -w /src szabgab/rust:latest /home/ubuntu/.cargo/bin/cargo build --release
```

## Upload to the server

```
scp target/release/echo-get demo@IP:app/axum-demo
```

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

In the service configuration file search for the word `demo` and `axum` and replace them with the
values appropriate to your setup.

{% embed include file="src/examples/deploy/axum-demo.service" %}

* Upload the `axum-demo.service` file to `/etc/systemd/system/axum-demo.service`

```
$ scp axum-demo.service root@IP:/etc/systemd/system/axum-demo.service
```

Connect again to the server as user `root` and set up the service.

```
$ ssh root@IP
```

```
# systemctl daemon-reload
# systemctl enable axum-demo.service
# systemctl start axum-demo.service
```

## Verify that the application runs as a service:

Running `curl http://localhost:3000` again (on the server) should return the page again.


## nginx

After replacing `demo` in the file, upload the nginx configuration file:

```
scp axum-demo.code-maven.com     root@IP:/etc/nginx/sites-available/
```

```
# cd /etc/nginx/sites-enabled
# ln -s /etc/nginx/sites-available/axum-demo.code-maven.com
# systemctl restart nginx
```

{% embed include file="src/examples/deploy/axum-demo.code-maven.com" %}

## DNS name resolving

If you have a domain you can map any hostname to this IP. I use the [iwantmyname](https://iwantmyname.com/en/) service to register domain names and handle DNS configuration. So I mapped `axum-demo.code-maven.com` to the IP addresses Linode gave me. (Both IPv4 and IPv6.)

Then waited a minute to allow for their service to be updated and then checked with [whatsmydns](https://www.whatsmydns.net) service if the new name already resolves by a large chunk of the Internet.

## HTTPS Certificate

* [Install certbot](https://certbot.eff.org/instructions?ws=nginx&os=snap) for Let's Encode certificate following the instructions in that link.

```
# snap install --classic certbot
# ln -s /snap/bin/certbot /usr/bin/certbot
# certbot --nginx
```
Domain name to certify: axum-demo.code-maven.com

## Done

At this point your service should be up and running. I'd recommend rebooting the server to verify that after a reboot the service runs again.

## Upgrade

After a while you will probably make some changes to the application and will want to upgrade it on the server.

* Build a new release
* Create a copy of the old executable so you can easily switch back if necessary.
* Upload the executable to the server replacing the old executable.
* Restart the service:

```
systemctl restart axum-demo
```

That's it.

