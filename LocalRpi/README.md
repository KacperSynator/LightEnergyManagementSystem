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
sudo apt-get install libboost-all-dev libbluetooth-dev cmake bluetooth pi-bluetooth bluez
```

#### 2. Install libblepp
```bash
git clone https://github.com/edrosten/libblepp.git
cd libblepp && mkdir build
cd build && cmake ..
sudo make install
```
#### 3. Install protobuf
Below code will install version 3.21.6 For latest version go to [Download protobuf](https://developers.google.com/protocol-buffers/docs/downloads). Pick version for `cpp` and with `tar.gz` extension.

```bash
wget https://github.com/protocolbuffers/protobuf/releases/download/v21.6/protobuf-cpp-3.21.6.tar.gz
tar –xvzf protobuf-cpp-3.21.6.tar.gz
cd protobuf-cpp-3.21.6
mkdir build && cd build
cmake -Dprotobuf_BUILD_TESTS=OFF ..
sudo make install
```

#### 4. Install protobuf-compiler (optional for .proto file modifcation)
```bash
sudo apt-get install protobuf-compiler
```

#### 4.5 Generate from .proto file
```bash
cd inc
protoc --cpp_out=. lamp_controller.proto
mv lamp_controller.pb.cc ../src
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
