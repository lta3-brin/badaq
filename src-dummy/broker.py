import socket
import threading
import time

from paho.mqtt import client as mqtt_client


def handle_client(socket, client, topic, idx):
    if idx == 1:  # sebagai publisher di mqtt
        while True:
            request = socket.recv(10000)

            client.publish(topic, request)

            if not request:
                break

    else:  # sebagai subscriber di mqtt

        def on_message(client, userdata, msg):
            socket.send(msg.payload)

        client.subscribe(topic)
        client.on_message = on_message
        client.loop_forever()


if __name__ == "__main__":
    bind_ip = "0.0.0.0"
    bind_port = 20508

    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.bind((bind_ip, bind_port))

    server.listen(5)

    print(f"Listening on port {bind_ip}:{bind_port}")
    port = 1883
    broker = "127.0.0.1"
    topic = "python/tcpbroker"
    client_id = mqtt_client.CallbackAPIVersion.VERSION2
    klient = mqtt_client.Client(client_id, client_id="tcp_broker")
    klient.connect(broker, port)

    idx = 1
    while True:
        client, addr = server.accept()
        print(f"Accepted connection from: {addr[0]}:{addr[1]}")

        client_handler = threading.Thread(
            target=handle_client,
            args=(client, klient, topic, idx),
        )
        client_handler.start()
        idx += 1
