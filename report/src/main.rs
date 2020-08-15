use anyhow::Result;
use bytes::{BufMut, BytesMut};
use std::{net::UdpSocket, process::Command};
use std::{thread::sleep, time::Duration};
use std::process::Output;

/// Measuring temperature
/// 
/// Due to the architecture of the SoCs used on the Raspberry Pi range, 
/// and the use of the upstream temperature monitoring code in the 
/// Raspberry Pi OS distribution, Linux-based temperature measurements 
/// can be inaccurate. There is a command that can provide an accurate 
/// and instantaneous reading of the current SoC temperature, as it 
/// communicates with the GPU directly:
///
/// vcgencmd measure_temp
fn temperature() -> Result<f32> {
    let Output { 
        stdout, 
        stderr: _, 
        status: _ 
    } = Command::new("vcgencmd")
        .arg("measure_temp")
        .output()?;
    let rate = String::from_utf8_lossy(&stdout)
        .split('=')
        .next_back()
        .unwrap_or("0'c")
        .split('\'')
        .next()
        .unwrap_or("0")
        .trim()
        .parse::<f32>()
        .unwrap_or(0.0);
    Ok(rate)
}

/// PWM速率计算
/// 
/// 根据温度计算PWM速率.
/// 当温度小于或等于40的时候为最低转速，
/// 当温度大于或者等于70的时候为最高转速，
/// 40-70之间按0-1023浮动.
fn pwm(temperature: f32) -> u16 {
    match temperature {
        x if x <= 40.0 => 0.0,
        x if x >= 70.0 => 1024.0,
        _ => (temperature - 40.0) * 34.13
    }.ceil() as u16
}

/// 编码数据包
/// 
/// 首位为1，表示包类型，
/// 这里为固定类型，
/// 后续两位为PWM速率.
fn encoder(rate: u16) -> BytesMut {
    let mut packet = BytesMut::new();
    packet.put_u8(1u8);
    packet.put_u16(rate);
    packet
}

/// 主循环
/// 
/// 每隔1s读取一次核心温度并计算PWM
/// 速率发送给远端单片机端口.
fn poll(socket: &mut UdpSocket, addr: &str) -> Result<()> {
    let packet = encoder(pwm(temperature()?));
    socket.send_to(&packet, addr)?;
    sleep(Duration::from_secs(1));
    Ok(())
}

/// 启动服务器
/// 
/// UDP端口本地绑定为随机分配.
fn main() -> Result<()> {
    let addr = "192.168.3.6:8088";
    let mut socket = UdpSocket::bind("0.0.0.0:0")?;
    loop { poll(&mut socket, addr)? }
}
