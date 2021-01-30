const ws = new WebSocket("ws://127.0.0.1:8080");

ws.addEventListener("open", event => {
	console.log("Sending message to server: Meow!");
	ws.send("Meow!");
});

ws.addEventListener("message", event => {
	console.log("Message from server: ", event.data);
});
