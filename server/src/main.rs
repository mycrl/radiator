use anyhow::Result;
use bytes::{BufMut, BytesMut};
use std::{fs, net::UdpSocket};
use std::{thread::sleep, time::Duration};

/// 远程地址
/// 温度报告文件
const ADDR: &str = "192.168.3.6:8088";
const ZONE_TEMP: &str = "/sys/class/thermal/thermal_zone0/temp";

/// 获取CPU温度
/// 
/// 读取文件将温度转为摄氏度.
fn temperature() -> Result<u16> {
    let vec  = fs::read(ZONE_TEMP)?;
    let template = String::from_utf8_lossy(&vec);
    let rate: u16 = template.trim().parse()?;
    Ok(rate / 1000)
}

/// PWM速率计算
/// 
/// 根据温度计算PWM速率.
/// 当温度小于或等于40的时候为最低转速，
/// 当温度大于或者等于70的时候为最高转速，
/// 40-70之间按0-1023浮动.
fn pwm(temperature: u16) -> u16 {
    if temperature <= 40 { return 0 }
    if temperature >= 70 { return 1024 }
    ((temperature as f32 - 40.0) * 34.13) as u16
}

/// 编码数据包
/// 
/// 首位为1，表示包类型，
/// 这里为固定类型，
/// 后续两位为PWM速率.
fn encoder(rate: u16) -> BytesMut {
    let mut packet = BytesMut::new();
    packet.put_u8(1);
    packet.put_u16(rate);
    packet
}

/// 主循环
/// 
/// 每隔1s读取一次核心温度并计算PWM
/// 速率发送给远端单片机端口.
fn poll(socket: &mut UdpSocket) -> Result<()> {
    let rate = pwm(temperature()?);
    let packet = encoder(rate);
    socket.send_to(&packet, ADDR)?;
    sleep(Duration::from_secs(1));
    Ok(())
}

/// 启动服务器
/// 
/// UDP端口本地绑定为随机分配.
fn main() -> Result<()> {
    let mut socket = UdpSocket::bind("0.0.0.0:0")?;
    loop { poll(&mut socket)? }
}
