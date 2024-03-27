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
Main.day_or_night = "night";
Main.phase = {
    name: "night",
    who_saying: 1,
    able_to_selecting: true,
}
Main.table_element = table_element;
Main.selectedPlayers = [];
Main.me = {
    role: "Mafia",
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


    let me = { name: "Akke" };
    Socket.send(me);


    Main.gameEvents.startGame({"role": "Mafia", "number": 1})
};

Main.gameEvents = {};

Main.gameEvents.startGame = function (data) {
    Main.me.role = data.role;
    Main.me.number = data.number;
    Main.players[Main.me.number].type = "me";
    Table.redrawTable();
    Table.showRole();
    setTimeout(Table.hideRole, 3000);
};

Main.gameEvents.startNight = function () {
    Main.day_or_night = "night";
    Main.phase = {
        name: "night",
        able_to_selecting: (Main.me.role == "Cityzen" ? false : true),
    }
    Table.redrawTable();
};

Main.gameEvents.startSunrise = function () {
    Main.day_or_night = "day";
    Main.phase = {
        name: "sunrise",
        able_to_selecting: false,
    }
    Table.redrawTable();
};

Main.gameEvents.startSaying = function () {
    Main.day_or_night = "day";
    Main.phase = {
        name: "saying",
        able_to_selecting: false,
    }
    Table.redrawTable();
};

Main.gameEvents.startVoting = function () {
    Main.day_or_night = "day";
    Main.phase = {
        name: "voting",
        target: 0,
        able_to_selecting: true,
    }
};

Main.gameEvents.votingFor = function (number) {
    Main.phase.target = Main.players[number];
    Table.redrawTable();
}

Main.gameEvents.startSunset = function () {
    Main.day_or_night = "day";
    Main.phase = {
        name: "sunset",
        target: 0,
        able_to_selecting: false,
    }
    Table.redrawTable();
};


Main.playersEvents = {};

Main.playersEvents.okPress = function() {
    if (Main.phase.able_to_selecting) {
        if (Main.day_or_night == "day") {
            if (Main.phase.name == "saying") {
                if (Main.phase.who_saying == Main.me.number) {
                    Socket.pickPlayers(Main.selectedPlayers);
                }
                Main.phase.able_to_selecting = false;
            }
            Main.gameEvents.startNight();
        } else if (Main.day_or_night == "night") {
            if (Main.me.role != "Cityzen") {
                Socket.pickPlayers(Main.selectedPlayers);
                Main.phase.able_to_selecting = false;
            }
            Main.gameEvents.startVoting();
            Main.gameEvents.votingFor(1);
        }
    }
    Main.selectedPlayers = [];
}

export default Main;
