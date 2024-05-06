# Reflection
<hr>

## 2.1. Original code of broadcast chat
### 1st Client POV:
![img.png](img.png)
### 2nd Client POV:
![img_1.png](img_1.png)
### 3rd Client POV:
![img_2.png](img_2.png)
### Server POV:
![img_3.png](img_3.png)
To run this program what we need first is to establish the connection by running the server first. The server will then listen from the port `2000` and awaits any connection from a client side, in this example I ran 3 clients to connect to the server, thus the server shows three new connections coming from those 3 new clients. Then the client can simply type in any form of message, the server will broadcast that output to itself and the other two clients too. Hence, after inputting three messages, each of the client has three broadcast output from the server.
## 2.2 Modifying the websocket port
To make sure the program still runs when the port is changed to `8080`, we need to modify the websocket for both the client and the server. Both the client and server also uses the same websocket of `tokio_websocket`. Here is where we need to modify it and what line shows that both server and client uses the `tokio_websocket` as their websocket
### where to modify server
![img_4.png](img_4.png)
### which line uses `tokio_websocket` in server
![img_5.png](img_5.png)
### where to modify client
![img_6.png](img_6.png)
### which line uses `tokio_websocket` in client
![img_7.png](img_7.png)