use amiquip::{Connection, ConsumerMessage, ConsumerOptions, ExchangeDeclareOptions, QueueDeclareOptions};

fn main() -> amiquip::Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = connection.open_channel(None)?;

    let exchange = channel.exchange_declare(
        amiquip::ExchangeType::Direct,
        "insight_actions_exchange",
        ExchangeDeclareOptions::default(),
    )?;

    let queue = channel.queue_declare("insight_actions_queue", QueueDeclareOptions::default())?;

    queue.bind(&exchange, "routing_key", Default::default())?;

    let consumer = queue.consume(ConsumerOptions::default())?;

    println!("Press Ctrl+C to stop consumer");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) Received [{}]", i, body);
                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}
