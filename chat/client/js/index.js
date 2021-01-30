document.addEventListener("DOMContentLoaded", () => {
	const username = prompt("What is your name?");
	document.getElementById("username").innerText = 	`User: ${username}`;
	const socket = new WebSocket('ws://127.0.0.1:8080');
	socket.onmessage = event => {
		// # message is the <textarea/>
		const messages = document.getElementById("messages");
		// Append the received message
		// to the existing list of messages
		const msg = JSON.parse(event.data);
		const time = (new Date(Number(msg.received_at)))
			.toLocaleString("en-US")
		messages.value += `[${time}] ${msg.name}: ${msg.message}\n`
	}
	const sendButton = document.getElementById("send");
	sendButton.addEventListener("click", event => {
		// message is the <input>
		const message = document.getElementById("message");
		socket.send(JSON.stringify({
			name: username,
			message: message.value
		}));
		message.value = ""; // Clear the input box
	});
});

