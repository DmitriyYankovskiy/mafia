import Main from "./main.js";

let ws;

let Socket = {};

Socket.onmessage = function(event) {
    if (Main.phase == "starting") {
        Main.startGame(event.data);
    }
}

Socket.setOnOpen = function(event) {
    ws = new WebSocket(`ws://${location.hostname}:9999/ws`);
    ws.onopen = event;
}

Socket.init = function () {
    ws.onmessage = onMessage;
}

Socket.send = function(obj) {
    ws.send(JSON.stringify(obj));
}


Socket.pickPlayers = function(selectedPlayers) {
    if (selectedPlayers.length == 0) {
        Socket.send({
            vote: 0,
        });
    } else {
        Socket.send({
            vote: selectedPlayers[0].number,
        });
    }
}

Socket.startGame = function() {
    return new Promise(function(resolve, reject) {
        ws.send(JSON.stringify({name: (prompt("name:", "Player") || "Player")}));
        ws.onmessage = function(e) {
            resolve(JSON.parse(e.data));
        };
        //resolve({"role": "Mafia", "number": 7, "countPlayers": 10});
        ws.onmessage = Socket.onmessage;
    });
};

Socket.nextPhase = function(data) {

};

export default Socket;