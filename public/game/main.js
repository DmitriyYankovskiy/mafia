import Table from "./table.js";
import Socket from "./websocket.js";

let tableElement = document.getElementById("round-table");

function Player(number) {
    this.number = number;
    this.state = "alive";
    this.element = document.getElementById(`player-${number}`);
    this.type = "";
}
let Main = {};
Main.players = {};
Main.dayOrNight = "night";
Main.phase = {
    name: "night",
    ableToSelecting: true,
}
Main.tableElement = tableElement;
Main.selectedPlayers = [];
Main.me = {
    role: "Mafia",
    player: {},
}
Main.init = function() {
    let j = 1;
    for (let i of tableElement.childNodes) {
        if (i.id == `player-container-${j}`) {
            this.players[j] = new Player(j);
            j++;
        }
    }
    Socket.init();
    Table.init();
    Socket.startGame().then(Main.gameEvents.startGame);
};

Main.gameEvents = {};

Main.gameEvents.startGame = function (data) {
    Main.me.role = data.role;
    Main.me.player = Main.players[data.number];
    Main.players[Main.me.player.number].type = "me";
    Table.redrawTable();
    Table.showRole();
    setTimeout(Table.hideRole, 3000);
};

Main.gameEvents.startNight = function () {
    Main.dayOrNight = "night";
    Main.phase = {
        name: "night",
        ableToSelecting: (Main.me.role == "Cityzen" ? false : true),
    }
    Table.redrawTable();
};

Main.gameEvents.startSunrise = function () {
    Main.dayOrNight = "day";
    Main.phase = {
        name: "sunrise",
        ableToSelecting: false,
    }
    Table.redrawTable();
};

Main.gameEvents.startSaying = function () {
    Main.dayOrNight = "day";
    Main.phase = {
        name: "saying",
        targets: [],
        whoSaying: {},
        ableToSelecting: false,
    }
    Table.redrawTable();
};

Main.gameEvents.saying = function (say) {
    Main.phase.whoSaying = say;
    if (say == Main.me.player) {
        Main.phase.ableToSelecting = true;
    } else {
        Main.phase.ableToSelecting = false;
    }
    Table.redrawTable();
};

Main.gameEvents.addTarget = function (player) {
    Main.phase.targets.push(player);
    Table.redrawTable();
};

Main.gameEvents.startVoting = function (targets) {
    Main.dayOrNight = "day";
    Main.phase = {
        name: "voting",
        targets: targets,
        target: 0,
        ableToSelecting: true,
    }
    for (let i in Main.phase.targets) {
        if (Main.phase.targets[i].type != "dead") Main.phase.targets[i].type = "target";
    }
};

Main.gameEvents.votingFor = function (number) {
    for (let i in Main.phase.targets) {
        if (Main.phase.targets[i].type != "dead") Main.phase.targets[i].type = "target";
    }
    Main.phase.target = Main.phase.targets[number];
    Main.phase.target.type = "targetNow";
    Table.redrawTable();
};

Main.gameEvents.startSunset = function () {
    for (let i in Main.phase.targets) {
        if (Main.phase.targets[i].type != "dead") Main.phase.targets[i].type = "alive";
    }
    Main.dayOrNight = "day";
    Main.phase = {
        name: "sunset",
        ableToSelecting: false,
    }
    Table.redrawTable();
};

Main.gameEvents.killPlayer = function (player) {
    player.state = "dead";
} 


Main.playersEvents = {};

Main.playersEvents.okPress = function() {
    if (Main.phase.ableToSelecting && Main.me.player.state != "dead") {
        if (Main.dayOrNight == "day") {
            if (Main.phase.name == "saying") {
                Socket.pickPlayers(Main.selectedPlayers);
                Main.phase.ableToSelecting = false;
                /*? Main.selectedPlayers.push(Main.me.player);*/
                /*? Main.selectedPlayers.push(Main.players[3]);*/
                /*?*/ Main.gameEvents.startVoting(Main.selectedPlayers);
                /*?*/ Main.gameEvents.votingFor(0);
            } else if (Main.phase.name == "voting") {
                if (Main.selectedPlayers.length != 0 && Main.phase.target == Main.selectedPlayers[0]) {
                    Socket.pickPlayers(Main.selectedPlayers);
                    Main.phase.ableToSelecting = false;

                    /*?*/ Main.gameEvents.startSunset();
                    /*?*/ setTimeout(Main.gameEvents.startNight, 1000);
                }
            }
        } else if (Main.dayOrNight == "night") {
            if (Main.me.role != "Cityzen") {
                Socket.pickPlayers(Main.selectedPlayers);
                Main.phase.ableToSelecting = false;

                /*?*/ Main.gameEvents.startSaying(Main.me.player);
                /*?*/ Main.gameEvents.saying(Main.me.player);
                /*?*/ if (Main.selectedPlayers.length != 0) Main.gameEvents.killPlayer(Main.selectedPlayers[0])
            }
        }
    }
    Main.selectedPlayers = [];
}

export default Main;
