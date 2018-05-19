# SFTP

To open SFTP session use command:

```bash
sftp username@remote_hostname
```

If working on custom ssh port (not 22) then connection can be opened in two ways:
```bash
sftp -oPort=custom_port username@remote_host  # or 
sftp -P <port_num> username@remote_host
```

## Navigating with SFTP

Navigation on **remote filesystem** can be done similarly to the regular linux commands: ``cd``, ``ls``, ``pwd``, etc.

We can direct commands towards the **local filesystem** preceeding all commands with *l* for *local*.

```bash
sftp> lpwd
Local working directory: /home/umat
sftp> lcd Documents/
sftp> lpwd
Local working directory: /home/umat/Documents
```

## Transfering files with SFTP

### Transfering remote files to the local system

```bash
get remotefile  # download remote file and save it with the same name
get remotefile localfile # download remote file specifying its name on local filesystem
get -r someDirectory # copy directory and all of its contents
```

### Transfering Local files to Remote System

```bash
put localFile
put -r localDirectory
```