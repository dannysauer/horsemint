extern crate config;

use rumqttc::{MqttOptions, Client, QoS};
//use std::collections::HashMap;
//use std::time::Duration;
//use std::thread;
//use std::io::{ErrorKind};

fn main() {
    let mut settings = config::Config::default();
    settings
        // config files
        //.merge(config::File::with_name("/etc/horsemint/horsemint")).unwrap()
        .merge(config::File::with_name("Settings")).unwrap()
        // environment vars starting with $HORSEMINT_
        .merge(config::Environment::with_prefix("HORSEMINT")).unwrap();
    let mut mqttoptions = MqttOptions::new(
        settings.get_str("mqtt_channel").unwrap_or_default(),
        settings.get_str("mqtt_host").unwrap_or_default(),
        settings.get_int("mqtt_port").unwrap_or_default() as u16,
    );
    mqttoptions.set_keep_alive(5);
    mqttoptions.set_credentials(
        settings.get_str("username").unwrap_or_default(),
        settings.get_str("password").unwrap_or_default(),
    );

    let (mut client, mut connection) = Client::new(mqttoptions, 10);
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).unwrap();

    let i = 0;
    // thread::spawn(move || for i in 0..10 {
    client
        .publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize])
        .unwrap();
    //    thread::sleep(Duration::from_millis(100));
    //});


    /*
     * Not running:
     *  Notification = Err(Io(Os {
     *    code: 111, kind: ConnectionRefused, message: "Connection refused" }))
     *
     * Bad auth:
     *  Notification = Err(Io(Custom {
     *    kind: InvalidData,
     *    error: "Broker rejected connection. Reason = NotAuthorized" }))
     *  Notification = Err(Io(Custom {
     *    kind: ConnectionReset, error: "connection reset by peer" }))
     *
     * Ok:
     *  Notification = Ok((Some(Connected), None))
     *  Notification = Ok((None, Some(Subscribe(1))))
     *  Notification = Ok((None, Some(Publish(2))))
     *  Notification = Ok((Some(SubAck(SubAck {
     *    pkid: 1, return_codes: [Success(AtMostOnce)] })), None))
     *  Notification = Ok((Some(PubAck(PubAck { pkid: 2 })), None))
     *  Notification = Ok((Some(Publish(
     *    Topic = hello/rumqtt, Qos = AtMostOnce,
     *    Retain = false, Pkid = 0, Payload Size = 0)), None))
     *  Notification = Ok((None, Some(PingReq)))
     *  Notification = Ok((Some(PingResp), None))
     *  Notification = Ok((None, Some(PingReq)))
     *  Notification = Ok((Some(PingResp), None))
     */
    for (_i, notification) in connection.iter().enumerate() {
        if notification.is_err() {
            let n = notification.err();
            println!("Failure Notification = {:?}", n);
            /*
             * TODO: figure out how to get error kind :/
             *
            match notification.unwrap_err() {
                //std::io::ErrorKind::ConnectionRefused => println!("refused"),
                rumqttc::ConnectionError::Io() => println!("refused"),
                _ => println!("kapow"),
            }
            */
            /*
            if notification.unwrap_err().kind()
                == std::io::ErrorKind::ConnectionRefused
            {
                println!("refused: {:?}", notification);
            }
            */
            break;
        } else {
            // success!
            let n = notification.ok().unwrap_or_default();
            println!("Notification n = {:?}", n);
        }
    }
}
