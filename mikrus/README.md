# Nginx Reverse Proxy for Multiple Applications
This repository contains a configuration for an Nginx reverse proxy that allows you to host multiple applications on the same VPS server. 
To run the nginx server, you need to have Docker and Docker Compose installed on your VPS.
```
docker compose up -d
```

# Backup commands (for nginx)

Stop nginx
```
sudo systemctl stop nginx
```
Start nginx
```
sudo systemctl start nginx
```

Kill vscode-server processes
```
pkill -f vscode-server
```

# Adding new application
* Add new application to `nginx/sites-enabled`
* Configure the DNS record on CloudFlare
Example can be seen in the image below:
![CloudFlare DNS](img/cloudflare-dns.png)
