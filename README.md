# Rust Learning
This repository is dedicated to tracking my progress of learning Rust. As of writing this, I'm using Actix as a web server. While Actix is a fast library (one of the fastest, express.js wimpers in comparison) I'm here to learn. Due to that, once I move forward in reading documentation and the book, the web server will switch over to my own source.

# Plans
I want my final project to culminate in a chat application. When a user connects to a chat, they will open up a webhook connection that the Rust server will store in memory. Every time a message is sent to the server, the server emits the change to all of the clients apart from the client that sent the change. A rinse and repeat idea, but it effectively helps teach a lot of different concepts.