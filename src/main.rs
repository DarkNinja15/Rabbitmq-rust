use amiquip::{Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish};

fn main()->amiquip::Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = connection.open_channel(None)?;

    let exchange = Exchange::direct(&channel);
    let queue = channel.queue_declare("hello", Default::default())?;

    exchange.publish(Publish::new(b"hello world", "hello"))?;

    let consumer = queue.consume(ConsumerOptions::default())?;

    println!("Press Ctrl+C to stop consumer");

    for(i,message)in consumer.receiver().iter().enumerate(){
        match message {
            ConsumerMessage::Delivery(delivery)=>{
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) Received [{}]", i, body);
                consumer.ack(delivery)?;
            }
            other =>{
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}
