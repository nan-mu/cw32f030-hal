# cw32f030 hal

使用[svd2rust](https://docs.rs/svd2rust/latest/svd2rust/)工具生成的项目，计划在此基础上编写可用的hal库（非官方）

编译固件

```shell
cargo build --release
cargo objcopy --release -- -O binary target/firmware.bin
```
