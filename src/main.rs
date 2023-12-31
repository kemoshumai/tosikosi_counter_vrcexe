use std::{net::{SocketAddrV4, UdpSocket, Ipv4Addr}, time::Duration, thread, io};

use chrono::{Local, Datelike, Timelike};
use rosc::{encoder, OscMessage, OscType, OscError};

fn main() -> anyhow::Result<()> {
    println!("年越しのやつ! by Kemoshumai!!!!!!!!!!");
    
    let localhost = Ipv4Addr::new(127, 0, 0, 1);

    let host_address = SocketAddrV4::new(localhost, 9002);
    let vrchat_address = SocketAddrV4::new(localhost, 9000);
    
    let socket = UdpSocket::bind(host_address)?;

    let send = move |msg_buf: &Vec<u8>| -> io::Result<usize> {
        socket.send_to(&msg_buf, vrchat_address)
    };

    loop {
        let now = Local::now();
        let target = now.clone().with_month(12).unwrap().with_day(31).unwrap().with_hour(23).unwrap().with_minute(59).unwrap().with_second(59).unwrap();
        let distance = target - now + chrono::Duration::seconds(1);

        let (count, is_min) = 
            if distance.num_seconds() < 1000
            {
                // 1000秒未満は秒で表示
                (
                    distance.num_seconds(),
                    false
                )
            }
            else if distance.num_minutes() < 1000
            {
                // 1000分未満は分で表示
                (
                    distance.num_minutes(),
                    true
                )
            }
            else
            {
                // それ以外(年越し後も)は0秒で表示
                (
                    0, false
                )
            };

        send(&make_counter_osc_message_buf(count.try_into()?)?)?;
        send(&make_is_min_osc_message_buf(is_min)?)?;

        println!("{}秒({}分) [{}, {}]", distance.num_seconds(), distance.num_minutes(), count, is_min);

        thread::sleep(Duration::from_millis(500));
    }
}


fn make_counter_osc_message_buf(count: u16) -> Result<Vec<u8>, OscError>{
    encoder::encode(&rosc::OscPacket::Message(OscMessage{
        addr: "/avatar/parameters/KEMOSHUMAI_TSKS_CNT".to_string(),
        args: vec![OscType::Float(count as f32 / 1000.0)],
    }))
}

fn make_is_min_osc_message_buf(is_min: bool) -> Result<Vec<u8>, OscError>{
    encoder::encode(&rosc::OscPacket::Message(OscMessage{
        addr: "/avatar/parameters/KEMOSHUMAI_TSKS_MIN".to_string(),
        args: vec![OscType::Bool(is_min)],
    }))
}
