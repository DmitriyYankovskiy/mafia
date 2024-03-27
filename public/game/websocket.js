import Main from "./main.js";

let ws = new WebSocket(`ws://${location.hostname}:9999/ws`);

function onMessage(event) {
    if (Main.phase == "starting") {
        Main.startGame(event.data);
    }
}
let Socket = {};
Socket.init = function () {
    ws.onmessage = onMessage;
    ws.onopen = function () {
        let me = { name: "Akke" };
        Socket.send(me);
    }
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

export default Socket;