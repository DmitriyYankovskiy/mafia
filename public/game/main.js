import Table from "./table.js";
import Socket from "./websocket.js";

let tableDiv = document.getElementById("round-table");
function Player(number) {
    this.number = number;
    this.state = "alive";
    this.element = document.getElementById(`player-${number}`);
    this.type = ""
}
let Main = {};
Main.players = {};
Main.phase = "starting";
Main.tableDiv = tableDiv;
Main.me = {
    role: "citizen",
    number: 1
}
Main.init = function() {
    for (let i = 1; i <= tableDiv.childElementCount; i++) {
        this.players[i] = new Player(i);
    }
    Socket.init();
    Table.init();
    this.startGame({"role": "a", "number": 1})
};
Main.startGame = function (data) {
    this.me.role = data.role;
    this.me.number = data.number;
    this.players[this.me.number].type = "me";
    Table.redrawTable();
    Table.showRole();
};

export default Main;
