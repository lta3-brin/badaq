import socket
import threading
import time


def handle_client(socket):
    with open("run.txt", "r") as file:
        for line in file:
            message = line.strip().encode()
            print(message)
            socket.send(message)
            time.sleep(1)
            # input("")

        socket.close()


if __name__ == "__main__":
    bind_ip = "0.0.0.0"
    bind_port = 20508

    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.bind((bind_ip, bind_port))

    server.listen(5)

    print(f"Listening on port {bind_ip}:{bind_port}")
    while True:
        client, addr = server.accept()
        print(f"Accepted connection from: {addr[0]}:{addr[1]}")

        client_handler = threading.Thread(
            target=handle_client,
            args=(client,),
        )
        client_handler.start()
