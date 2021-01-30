const ws = new WebSocket("ws://127.0.0.1:8080");

let sendButton = document.getElementById("send");
let message = document.getElementById("message");
let messages = document.getElementById("messages");

ws.addEventListener("open", event => {
	sendButton.addEventListener('click', event=> {
		let time = new Date();
		let messageText= `${time.getHours()}:${time.getMinutes()} me: ${message.value}`;
		ws.send(message.value);
		messages.value = messages.value.length > 0 ?  messages.value + '\n' + messageText : messageText;
	});
});

ws.addEventListener("message", event=> {
	let time = new Date();
	let serverMsg = `${time.getHours()}:${time.getMinutes()} server: ${event.data}`;
	messages.value = messages.value.length > 0 ?  messages.value + '\n' + serverMsg : serverMsg;
})
