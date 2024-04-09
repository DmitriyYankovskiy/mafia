import Main from "./main.js";

let ws;

let Socket = {};

Socket.onmessage = function(event) {
    console.log(JSON.parse(event.data));
    let data = JSON.parse(event.data);
    if (Main.phase == "starting") {
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