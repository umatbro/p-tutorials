Go to AUR page, example for [teamviewer](https://aur.archlinux.org/packages/teamviewer/).

Copy git repository address and clone it:

```
git clone https://aur.archlinux.org/teamviewer.git
```
Go to the repository directory with `cd teamviewer`. Once in there, run following commands:

```
makepkg -s
sudo pacman -U <name_of_package>.pkg.tar.xz
```
