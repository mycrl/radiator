# Radiator

获取树莓派Soc温度并通过`PWM`引脚发送占空比控制风扇转速.

![device](./device.jpg)


### 温度控制策略

- 当温度`<= 40`度时，风扇保持最低转速.
- 当温度`>= 60`度时，风扇达到最高转速.
- 树莓派自身温度策略为`60+`之后开始降频，所以这里的目的是尽量让树莓派保持最佳性能.


### 安装

注意: 请将风扇连接到硬件PWM引脚:
![GPIO-Pinout.png](./GPIO-Pinout.png)

然后你需要通过环境变量指定风扇PWM引脚和工作周期(秒):

```sh
export RADIATOR_PIN=12
export RADIATOR_DELAY=10
```

为了避免树莓派每次重启之后都需要手动启动进程的问题，
你可以使用自动化脚本安装服务:

```sh
./install.sh
systemctl status radiator.service
```

服务将自动安装并保持开机自动启动.


### License
[MIT](./LICENSE)
Copyright (c) 2020 Mr.Panda.
