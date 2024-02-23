import Main from "./main.js";

let players;
let table_element;

function getWidth(element) {
    return window.getComputedStyle(element).width.split("p")[0];
}

function playerClickListener(e) {
    let number = e.target.id.split("-");
    number = number[number.length - 1];
    for (let i in players) {
        if (players[i].number == number) {
            players[i].element.classList.add("selected-player");
            players[i].element.classList.remove("unselected-player");
            Main.selectedPlayers = [];
            Main.selectedPlayers.push(players[i]);
        } else {
            players[i].element.classList.remove("selected-player");
            players[i].element.classList.add("unselected-player");
        }
    }
    Table.redrawTable();
}
let Table = {};
Table.background_element = document.getElementById("background");

Table.redrawTable = function() {
    if (players.length == 0) {
        return 0;
    }
    let alivePlayers = [];
    for (let i in players) {
        if (players[i].state == "alive") {
            alivePlayers.push(players[i]);
        }
        players[i].element.display = "none";
    }
    let aliveCount = alivePlayers.length;
    let tableSize = getWidth(table_element);
    let deltaAngle = Math.PI * 2 / aliveCount;
    for (let i = 0; i < aliveCount; i++) {
        let playerSize = getWidth(alivePlayers[i].element.parentElement);
        alivePlayers[i].element.parentElement.style.top = `${-Math.sin(1.5 * Math.PI - deltaAngle * i) * tableSize / 2 + tableSize / 2 - playerSize / 2}px`;
        alivePlayers[i].element.parentElement.style.left = `${Math.cos(1.5 * Math.PI - deltaAngle * i) * tableSize / 2 + tableSize / 2 - playerSize / 2}px`;
        if (alivePlayers[i].type == "me") {
            alivePlayers[i].element.classList.add("me-player");
        }
    }
}

Table.init = function () {
    players = Main.players;
    table_element = Main.table_element;
    for (let i in players) {
        players[i].element.addEventListener("click", playerClickListener);
    }
    this.background_element.addEventListener("click", function (e) {
        if (Main.day_or_night == "day") {
            Main.gameEvents.startNight();
        } else {
            Main.gameEvents.startDay();
        }
    });
    this.redrawTable();
}

Table.showRole = function () {
    
}

Table.gameEvents = {};

Table.gameEvents.startNight = function () {
    Table.background_element.classList.remove("background-night");
    Table.background_element.classList.add("background-day");
}

Table.gameEvents.startDay = function () {
    Table.background_element.classList.add("background-night");
    Table.background_element.classList.remove("background-day");
}
export default Table;