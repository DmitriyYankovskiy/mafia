import Main from "./main.js";

let players;
let tableElement;

function getWidth(element) {
    return window.getComputedStyle(element).width.split("p")[0];
}

function playerClickListener(e) {
    if (Main.phase.ableToSelecting == 0 || Main.me.player.state.has("dead")) return;
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

Table.redrawTable = function() {
    if (players.length == 0) {
        return;
    }

    Table.backgroundElement.classList.remove("background-day");
    Table.backgroundElement.classList.remove("background-night");
    if (Main.dayOrNight == "day") {
        Table.backgroundElement.classList.add("background-day");
    } else {
        Table.backgroundElement.classList.add("background-night");
    }


    let alivePlayers = [];
    for (let i in players) {
        if (!players[i].state.has("dead")) {
            alivePlayers.push(players[i]);

            if (players[i].state.has("voted")) {
                players[i].element.classList.add("voted-player");
            } else {
                players[i].element.classList.remove("voted-player");
            }


            if (players[i].state.has("saying")) {
                players[i].element.classList.add("saying-player");
            } else {
                players[i].element.classList.remove("saying-player");
            }
            
            if (players[i].cnowedRole == 1) {
                players[i].element.classList.add("badrole-player");
            } else if (players[i].cnowedRole == 2) {
                players[i].element.classList.add("goodrole-player");
            }

        } else {
            players[i].element.style.display = "none";
            players[i].element.parentElement.style.display = "none";
        }
        if (!players[i].state.has("dead") && Main.phase.name == "voting" && (players[i].type == "target" || players[i].type == "targetNow")) {
            players[i].voicesCounterElement.classList.remove("hidden-span");
        } else {
            players[i].voicesCounterElement.classList.add("hidden-span");
        }
    }

    if (!Main.phase.ableToSelecting || Main.me.player.state.has("dead")) {
        Table.okElement.classList.add("invisible");
    } else {
        if (Main.phase.name == "discussion") {
            Table.okSpanElement.innerHTML = "Put it up";
        } else if (Main.phase.name == "voting") {
            Table.okSpanElement.innerHTML = "Vote";
        } else if (Main.phase.name = "night") {
            if (Main.me.role == "Mafia") Table.okSpanElement.innerHTML = "Shoot";
            if (Main.me.role == "Maniac") Table.okSpanElement.innerHTML = "Kill";
            if (Main.me.role == "Sheriff") Table.okSpanElement.innerHTML = "Check";
            if (Main.me.role == "Doctor") Table.okSpanElement.innerHTML = "Heal";
        }
        if (Main.selectedPlayers.length) {
            Table.okElement.classList.add("ok-button-clickable");
        } else {
            Table.okElement.classList.remove("ok-button-clickable");
        }
        Table.okElement.classList.remove("invisible");
    }


    let aliveCount = alivePlayers.length;
    let tableSize = getWidth(tableElement);
    let deltaAngle = Math.PI * 2 / aliveCount;
    for (let i = 0; i < aliveCount; i++) {
        let playerSize = getWidth(alivePlayers[i].element.parentElement);
        alivePlayers[i].element.parentElement.style.top = `${-Math.sin(1.5 * Math.PI - deltaAngle * i) * tableSize / 2 + tableSize / 2 - playerSize / 2}px`;
        alivePlayers[i].element.parentElement.style.left = `${Math.cos(1.5 * Math.PI - deltaAngle * i) * tableSize / 2 + tableSize / 2 - playerSize / 2}px`;
        if (alivePlayers[i].type == "me") {
            alivePlayers[i].element.classList.add("me-player");
        }
        if (alivePlayers[i].type == "targetNow") {
            alivePlayers[i].element.classList.add("target-now-player");
        } else {
            alivePlayers[i].element.classList.remove("target-now-player");
        }
        if (alivePlayers[i].type == "target") {
            alivePlayers[i].element.classList.add("target-player");
        } else {
            alivePlayers[i].element.classList.remove("target-player");
        }
    }
}

Table.init = function () {
    Table.backgroundElement = document.getElementById("background");
    Table.roleSpanElement = document.getElementById("role-span");
    Table.showMyRoleElement = document.getElementById("show-my-role-button");
    Table.okSpanElement = document.getElementById("ok-span");
    Table.okElement = document.getElementById("ok-button");
    Table.roleSpanElement.innerHTML = `Your role: ${Main.me.role}`;
    //console.log(9);
    players = Main.players;
    tableElement = Main.tableElement;
    for (let i in players) {
        players[i].element.addEventListener("click", playerClickListener);
    }
    Table.okElement.addEventListener("click", function (e) {
        Main.playersEvents.okPress();
        for (let i in players) {
            players[i].element.classList.remove("selected-player");
            players[i].element.classList.remove("unselected-player");
            players[i].element.classList.add("unselected-player");
        }
        Table.redrawTable();
    });
    Table.showMyRoleElement.addEventListener("mousedown", function (e) {
        Table.showRole();
    });
    Table.showMyRoleElement.addEventListener("mouseup", function (e) {
        Table.hideRole();
    });
    Table.showMyRoleElement.addEventListener("mouseout", function (e) {
        Table.hideRole();
    });
    this.redrawTable();
}

Table.showRole = function () {
    Table.roleSpanElement.classList.remove("hidden-span");
}

Table.hideRole = function () {
    Table.roleSpanElement.classList.add("hidden-span");
}

Table.gameEvents = {};

export default Table;