# nginx

## basics

* install - ``apt-get install nginx``
* start - ``systemctl start nginx``
* stop - ``systemctl stop ngnix``
* ensure nginx to start on boot  ``systemctl enable nginx``

#### Default server root

The default server root directory is ``var/www/html``. Files placed in this directory will be served on the server. This location is specified in the default server block configuration file that ships with Nginx, which is located at ``/etc/nginx/sites-enabled/default``.

#### nginx global configuration

The main configuration file is located at ``/etc/nginx/nginx.conf``