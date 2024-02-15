import Main from "./main.js";

let ws = new WebSocket(`ws://${location.hostname}:9998`);

function onMessage(event) {
    if (Main.phase == "starting") {
        Main.startGame(event.data);
    }
}
let Socket = {};
Socket.init = function () {
    ws.onmessage = onMessage;
}

export default Socket;