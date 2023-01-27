# Device Service

## Build Azure IoT SDK for C

Run the following

```shell
git submodule update --init --recursive
```

This should add all the needed submodules for building the project.

Now run

```shell
mkdir build_cmake
cd build_cmake
cmake ..
cmake --build .
```

This should build the project and generate the relevant libraries. 
