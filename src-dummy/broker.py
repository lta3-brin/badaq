import socket
import threading


def handle_client(socket):
    while True:
        request = socket.recv(1024)

        if not request:
            break

        print(request.decode("utf-8"))
        socket.send(request)


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
