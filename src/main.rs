use rumqttc::{MqttOptions, Client, QoS};
//use std::time::Duration;
//use std::thread;
//use std::io::{ErrorKind};

fn main() {
    let mut mqttoptions = MqttOptions::new("rumqtt-sync-client", "localhost", 1883);
    mqttoptions.set_keep_alive(5);
    mqttoptions.set_credentials("health_user", "health_pass");

    let (mut client, mut connection) = Client::new(mqttoptions, 10);
    client.subscribe("hello/rumqtt", QoS::AtMostOnce).unwrap();

    let i = 0;
    // thread::spawn(move || for i in 0..10 {
    client.publish("hello/rumqtt", QoS::AtLeastOnce, false, vec![i; i as usize]).unwrap();
    //    thread::sleep(Duration::from_millis(100));
    //});


    /*
     * Not running:
     * Notification = Err(Io(Os { code: 111, kind: ConnectionRefused, message: "Connection refused" }))
     *
     * Bad auth:
     * Notification = Err(Io(Custom { kind: InvalidData, error: "Broker rejected connection. Reason = NotAuthorized" }))
     *  Notification = Err(Io(Custom { kind: ConnectionReset, error: "connection reset by peer" }))
     *
     *  Ok:
     *  Notification = Ok((Some(Connected), None))
     *  Notification = Ok((None, Some(Subscribe(1))))
     *  Notification = Ok((None, Some(Publish(2))))
     *  Notification = Ok((Some(SubAck(SubAck { pkid: 1, return_codes: [Success(AtMostOnce)] })), None))
     *  Notification = Ok((Some(PubAck(PubAck { pkid: 2 })), None))
     *  Notification = Ok((Some(Publish(Topic = hello/rumqtt, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 0)), None))
     *  Notification = Ok((None, Some(PingReq)))
     *  Notification = Ok((Some(PingResp), None))
     *  Notification = Ok((None, Some(PingReq)))
     *  Notification = Ok((Some(PingResp), None))
     */
    for (_i, notification) in connection.iter().enumerate() {
        if notification.is_err() {
            println!("Notification = {:?}", notification);
            /*
             * TODO: figure out how to get error kind :/
             *
            match notification.unwrap_err() {
                //std::io::ErrorKind::ConnectionRefused => println!("refused"),
                rumqttc::ConnectionError::Io() => println!("refused"),
                _ => println!("kapow"),
            }
            */
            break;
           /* 
             if notification.unwrap_err().kind() == std::io::ErrorKind::ConnectionRefused {
                println!("refused: {:?}", notification);
            }
            */
        }
        else {
            // success!
        }
        println!("Notification = {:?}", notification);
    }

    println!("Hello, world!");
}
