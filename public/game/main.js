import Table from "./table.js";
import Socket from "./websocket.js";

let tableElement = document.getElementById("round-table");

function Player(number) {
    this.number = number;
    this.state = new Set();
    this.state.add("alive");
    this.element = document.getElementById(`player-${number}`);
    this.voicesCounterElement = document.getElementById(`player-voices-counter-${number}`);
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

Main.sendInformation = function() {
    Socket.startGame().then(Main.gameEvents.startGame);
}

Main.init = function() {
    Socket.setOnOpen(Main.sendInformation);
};

Main.gameEvents = {};

Main.gameEvents.startGame = function (data) {
    console.log(data);
    Socket.init();
    let j = 1;
    for (let i = 1; i <= data.cnt_characters; i++) {
        tableElement.innerHTML += `
        <div class="player-container" id="player-container-${i}">
            <div class="player unselected-player" id="player-${i}">
                <span class="player-number" id="player-number-${i}">
                    ${i}
                </span>
                <span class="player-voices-counter hidden-span" id="player-voices-counter-${i}">
                    0
                </span>
            </div>
        </div>
        `;
    }
    for (let i of tableElement.childNodes) {
        if (i.id == `player-container-${j}`) {
            Main.players[j] = new Player(j);
            j++;
        }
    }
    Main.me.role = data.role;
    Main.me.player = Main.players[data.num];
    Main.players[Main.me.player.number].type = "me";
    Table.init();
    Table.showRole();
    setTimeout(Table.hideRole, 3000);
};

Main.gameEvents.startNight = function () {
    Main.dayOrNight = "night";
    Main.phase = {
        name: "night",
        ableToSelecting: (Main.me.role == "Civilian" ? false : true),
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

Main.gameEvents.startDiscussion = function () {
    Main.dayOrNight = "day";
    Main.phase = {
        name: "discussion",
        whoDiscussion: {},
        ableToSelecting: false,
    }
    Table.redrawTable();
};

Main.gameEvents.nextDiscussioner = function (say) {
    Main.phase.whoDiscussion = say;
    for (let i in Main.players) {
        Main.players[i].state.delete("saying");
    }

    say.state.add("saying");
    if (say == Main.me.player) {
        Main.phase.ableToSelecting = true;
    } else {
        Main.phase.ableToSelecting = false;
    }
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
    Table.redrawTable();
};

Main.gameEvents.addVoice = function(player) {
    player.state.add("voted");
    player = Main.phase.target;
    player.voicesCounterElement.innerHTML = (Number(player.voicesCounterElement.innerHTML) + 1);
    Table.redrawTable();
}

Main.gameEvents.votingFor = function (player) {
    for (let i in Main.phase.targets) {
        if (Main.phase.targets[i].type != "dead") {
            Main.phase.targets[i].type = "target";
        }
    }

    for (let i in Main.players) {
        Main.players[i].state.delete("voted");
    }

    Main.phase.target = player;
    player.type = "targetNow";
    Table.redrawTable();
};

Main.gameEvents.startSunset = function () {

    for (let i in Main.players) {
        Main.players[i].state.delete("voted");
    }

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
    player.state.add("dead");
    Table.redrawTable();
} 

Main.gameEvents.addAccusion = function(player) {
    player.type = "target";
    Table.redrawTable();
}

Main.playersEvents = {};

Main.playersEvents.okPress = function() {
    if (Main.phase.ableToSelecting && !(Main.me.player.state.has("dead"))  && Main.selectedPlayers.length > 0) {
        if (Main.dayOrNight == "day") {
            if (Main.phase.name == "discussion") {
                Socket.accuse(Main.selectedPlayers[0]);
                Main.phase.ableToSelecting = false;
            } else if (Main.phase.name == "voting") {
                if (Main.selectedPlayers.length != 0 && Main.phase.target == Main.selectedPlayers[0]) {
                    Socket.vote();
                    Main.phase.ableToSelecting = false;
                }
            }
        } else if (Main.dayOrNight == "night") {
            if (Main.me.role != "Civilian") {
                Socket.action(Main.selectedPlayers[0]);
                Main.phase.ableToSelecting = false;
            }
        }
    }
    Main.selectedPlayers = [];
}

export default Main;
