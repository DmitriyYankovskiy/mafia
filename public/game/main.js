import Table from "./table.js";
import Socket from "./websocket.js";

let players = [];
let tableDiv = document.getElementById("round-table");;
let phase = "starting"

function Player(number) {
    this.number = number;
    this.state = "alive";
    this.element = document.getElementById(`player-${number}`);
}

function init() {
    for (let i = 1; i <= tableDiv.childElementCount; i++) {
        players.push(new Player(i));
    }
    Socket.init();
    Table.init();
}

let Main = {
    init: init,
    players: players,
    tableDiv: tableDiv,
}

export default Main;
