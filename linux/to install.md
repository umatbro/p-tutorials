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

## Other

Set proper timezone on your system

```bash
dpkg-reconfigure tzdata
```

Then select your timezone from one of available options.

# Arch linux

* [Installing teamviewer](https://linuxhint.com/install_teamviewer_arch_linux/)

## Installing from the AUR

**Instal the build essentials**. These are needed to compile packages on Arch Linux ARM.

```commandline
sudo pacman -S kernel26-headers file base-devel abs
```

Packages from [AUR](https://aur.archlinux.org/) can be installed *by hand* (manually).

To do that follow the steps given below:

* Download the tarball from the AUR preferably to ``~/Downloads`` (just click on the tarball from your browser and you should get it)

```commandline
# Make a sub-directory in downloads called builds:
mkdir ~/Downloads/builds
# Move the tarball to builds (foo is the name of the package you've downloaded): 
mv foo.tar.gz builds
# Change directory to the builds folder
cd builds
# Untar the tarball  
tar -xvf foo.tar.gz
# Move into the new sub-directory 
cd <foo>
```

To make/compile the package, run:

```
makepkg -s
```

This will build the package and pull in necessary dependencies as files. You're interested in the one that ends with .pkg.tar.xz (usually). 
The final event is running $sudo pacman -U on that file:

```
sudo pacman -U foo.pkg.xz

```
And you've done it...the safest way to install from the AUR. 
