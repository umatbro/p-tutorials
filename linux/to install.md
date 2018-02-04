# to install on clean Linux

Commands used on **debian-8.0-x86_64-minimal**.

## initial libs

```bash
apt-get updgrade && apt-get update
apt-get install screen curl sudo man git-core
apt-get install build-essential
```

### Setting system locale encoding

[Meaningful source](https://unix.stackexchange.com/a/269293).

To change system *locale* edit ``/etc/locale.gen`` file - uncomment encoding suitable for your system, say `en_US.UTF-8`.
Then run `locale-gen` as a root.

### Packages for building python

```bash
apt-get install libsqlite3-dev
apt-get install libssl-dev zlib1g-dev libncurses5-dev libncursesw5-dev libreadline-dev libsqlite3-dev
apt-get install libgdbm-dev libdb5.3-dev libbz2-dev libexpat1-dev liblzma-dev tk-dev
```

## Python3

Download python distro, in this example **3.7** beta version.

```bash
mkdir /home/downloads && cd "$_"
curl -O https://www.python.org/ftp/python/3.7.0/Python-3.7.0b1.tgz
tar -xzvf Python-3.x.xxx.tgz
```

Build Python (according to README.rst file). Also take a look at [this source](https://solarianprogrammer.com/2017/06/30/building-python-ubuntu-wsl-debian/).

```bash
cd Python-3.x.xxx
./configure --enable-optimizations
make
make test
make install
```
