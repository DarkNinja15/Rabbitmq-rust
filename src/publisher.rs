use amiquip::{Connection, Publish, ExchangeDeclareOptions, ExchangeType};

fn main() {
    let mut connection = Connection::insecure_open("amqp://localhost")
        .expect("Failed to connect to RabbitMQ");

    let channel = connection.open_channel(None)
        .expect("Failed to open a channel");

    let exchange = channel.exchange_declare(
        ExchangeType::Direct,
        "insight_actions_exchange",
        ExchangeDeclareOptions::default(),
    ).expect("Failed to declare exchange");

    exchange.publish(Publish::new("Hello, RabbitMQ!".as_bytes(), "routing_key"))
        .expect("Failed to publish message");

    exchange.publish(Publish::new("Rust and RabbitMQ".as_bytes(), "routing_key"))
        .expect("Failed to publish message");

    connection.close().expect("Failed to close connection");
}
