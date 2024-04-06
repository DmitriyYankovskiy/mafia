import Main from "./main.js";

let ws = new WebSocket(`ws://${location.hostname}:9999/ws`);

function onMessage(event) {
    if (Main.phase == "starting") {
        Main.startGame(event.data);
    }
}

function onOpen(event) {
    ws.onopen = event
}

let Socket = {};
Socket.init = function () {
    ws.onmessage = onMessage;
}

Socket.send = function(obj) {
    ws.send(JSON.stringify(obj));
}


Socket.pickPlayers = function(selectedPlayers) {
    if (selectedPlayers.length == 0) {
        Socket.send({
            vote: 0
        });
    } else {
        Socket.send({
            vote: selectedPlayers[0].number,
        });
    }
}

Socket.startGame = function() {
    return new Promise(function(resolve, reject) {
        ws.onopen = function() {
            console.log("pp");
            ws.send({name: "Aboba228"});
            ws.onmessage = function() {
                resolve({"role": "Mafia", "number": 6});
            };
            resolve({"role": "Mafia", "number": 7, "countPlayers": 10});
            ws.onmessage = Socket.onmessage;
        }
    });
};

Socket.nextPhase = function(data) {

};

export default Socket;