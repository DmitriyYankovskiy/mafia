import Main from "./main.js";

let ws;

let Socket = {};

Socket.onmessage = function(event) {
    console.log(JSON.parse(event.data));
    let data = JSON.parse(event.data);
    if (data.) {
        Main.startGame(event.data);
    }
}

Socket.setOnOpen = function(event) {
    ws = new WebSocket(`ws://${location.hostname}:9999/ws`);
    ws.onopen = event;
}

Socket.init = function () {
    ws.onmessage = Socket.onmessage;
}

Socket.send = function(obj) {
    ws.send(JSON.stringify(obj));
}


Socket.accuse = function(selectedPlayer) {
    Socket.send({
        Accuse: {
            target: selectedPlayer.number
        }
    });
}

Socket.action = function(selectedPlayer) {
    Socket.send({
        Action: {
            target: selectedPlayer.number
        }
    });
}

Socket.vote = function() {
    Socket.send({Vote: {}});
}

Socket.startGame = function() {
    return new Promise(function(resolve, reject) {
        ws.send(JSON.stringify({name: (prompt("name:", "Player") || "Player")}));
        ws.onmessage = (e) => {
            console.log(JSON.parse(e.data));
            resolve(JSON.parse(e.data));
            ws.onmessage = Socket.onmessage;
        };
    });
};

Socket.nextPhase = function(data) {

};

export default Socket;