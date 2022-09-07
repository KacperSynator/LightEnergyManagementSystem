# Local RPI

## Getting started

#### 0. Install gcc 11.3 (C++20)

Script can be modified to install other versinos of gcc. If using RPI other than **Pi4 in 32-bit** mode you must comment section `# Pi4 in 32-bit mode` and uncomment your device section.

```bash
./gcc11.sh
```
**It takes a lot of time to build, for RPI4 32bit 2GB it took about 10h**  

**Note:** if script fails with message like `no rule to make target install` replace `[Yy]* ) sudo make install ;;` with `[Yy]* ) sudo make ;;` then run script, after completion add `install` to script and run again (it worked for me).

New gcc compiler is installed in /usr/local/bin. You need to uninstall old gcc.
```bash
gcc -v  # check for gcc version
sudo apt-get --purge remove gcc-<your gcc version>
```


#### 1. Install dependencies
```bash
sudo apt-get install libboost-all-dev libbluetooth-dev cmake
```

#### 2. Install libblepp
```bash
git clone https://github.com/edrosten/libblepp.git
cd libblepp && mkdir build
cd build && cmake ..
sudo make install
```


### Errors 

**error while loading shared libraries: libble++.so.5: cannot open shared object file: No such file or directory**
```bash
sudo /sbin/ldconfig -v
```

**version GLIBCXX_3.4.29 not found** - override libstdc.so.6 library 
```bash
sudo ln -sf /usr/local/lib/libstdc++.so.6 /lib/arm-linux-gnueabihf/
```
