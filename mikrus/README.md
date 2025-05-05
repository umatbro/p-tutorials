# Backup commands

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
