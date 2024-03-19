import Main from "./main.js";

let players;
let table_element;

function getWidth(element) {
    return window.getComputedStyle(element).width.split("p")[0];
}

function playerClickListener(e) {
    if (Main.phase.able_to_selecting == 0) return;
    let number = e.target.id.split("-");
    number = number[number.length - 1];
    Main.selectedPlayers = [];
    for (let i in players) {
        if (players[i].number == number && players[i].element.classList.contains("unselected-player")) {
            players[i].element.classList.add("selected-player");
            players[i].element.classList.remove("unselected-player");
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
Table.role_span_element = document.getElementById("role-span");
Table.show_my_role_element = document.getElementById("show-my-role-button");
Table.ok_span_element = document.getElementById("ok-span");
Table.ok_element = document.getElementById("ok-button");

Table.redrawTable = function() {
    if (players.length == 0) {
        return;
    }

    Table.background_element.classList.remove("background-day");
    Table.background_element.classList.remove("background-night");
    if (Main.day_or_night == "day") {
        Table.background_element.classList.add("background-day");
    } else {
        Table.background_element.classList.add("background-night");
    }


    let alivePlayers = [];
    for (let i in players) {
        if (players[i].state == "alive") {
            alivePlayers.push(players[i]);
        }
        players[i].element.display = "none";
    }

    if (!Main.phase.able_to_selecting) {
        Table.ok_element.classList.add("invisible");
    } else {
        if (Main.phase.name == "saying") {
            Table.ok_span_element.innerHTML = "Put it up";
        } else if (Main.phase.name == "voting") {
            console.log(Main.phase);
            Table.ok_span_element.innerHTML = "Vote";
        } else if (Main.phase.name = "night") {
            if (Main.me.role == "Mafia") Table.ok_span_element.innerHTML = "Shoot";
            if (Main.me.role == "Maniac") Table.ok_span_element.innerHTML = "Kill";
            if (Main.me.role == "Sheriff") Table.ok_span_element.innerHTML = "Check";
            if (Main.me.role == "Doctor") Table.ok_span_element.innerHTML = "Heal";
        }
        Table.ok_element.classList.remove("invisible");
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
    Table.ok_element.addEventListener("click", function (e) {
        Main.playersEvents.okPress();
        for (let i in players) {
            players[i].element.classList.remove("selected-player");
            players[i].element.classList.remove("unselected-player");
            players[i].element.classList.add("unselected-player");
        }
        Table.redrawTable();
    });
    Table.show_my_role_element.addEventListener("mousedown", function (e) {
        Table.showRole();
    });
    Table.show_my_role_element.addEventListener("mouseup", function (e) {
        Table.hideRole();
    });
    Table.show_my_role_element.addEventListener("mouseout", function (e) {
        Table.hideRole();
    });
    this.redrawTable();
}

Table.showRole = function () {
    Table.role_span_element.classList.remove("hidden-span");
}

Table.hideRole = function () {
    Table.role_span_element.classList.add("hidden-span");
}

Table.gameEvents = {};

export default Table;