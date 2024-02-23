import Table from "./table.js";
import Socket from "./websocket.js";

let table_element = document.getElementById("round-table");

function Player(number) {
    this.number = number;
    this.state = "alive";
    this.element = document.getElementById(`player-${number}`);
    this.type = "";
}
let Main = {};
Main.players = {};
Main.day_or_night = "day";
Main.phase = "start-of-day";
Main.table_element = table_element;
Main.selectedPlayer = [];
Main.me = {
    role: "citizen",
    number: 1
}
Main.init = function() {
    let j = 1;
    for (let i of table_element.childNodes) {
        if (i.id == `player-container-${j}`) {
            this.players[j] = new Player(j);
            j++;
        }
    }
    Socket.init();
    Table.init();
    this.gameEvents.startGame({"role": "a", "number": 1})
};

Main.gameEvents = {};

Main.gameEvents.startGame = function (data) {
    Main.me.role = data.role;
    Main.me.number = data.number;
    Main.players[Main.me.number].type = "me";
    Table.redrawTable();
    Table.showRole();
};

Main.gameEvents.startNight = function () {
    Table.gameEvents.startNight();
    Main.day_or_night = "night";
};

Main.gameEvents.startDay = function () {
    Table.gameEvents.startDay();
    Main.day_or_night = "day";
};

export default Main;
