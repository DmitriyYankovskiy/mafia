import Main from "./main.js";

let players;
let tableDiv;

function getWidth(element) {
    return window.getComputedStyle(element).width.split("p")[0];;
}

function redrawTable() {
    if (players.length == 0) {
        return 0;
    }
    let alivePlayers = [];
    for (let i = 0; i < players.length; i++) {
        if (players[i].state == "alive") {
            alivePlayers.push(players[i]);
        }
        players[i].element.display = "none";
    }
    let aliveCount = alivePlayers.length;
    let tableSize = getWidth(tableDiv);
    let deltaAngle = Math.PI * 2 / aliveCount;
    for (let i = 0; i < aliveCount; i++) {
        let playerSize = getWidth(alivePlayers[i].element);
        alivePlayers[i].element.style.top = `${-Math.sin(1.5 * Math.PI - deltaAngle * i) * tableSize / 2 + tableSize / 2 - playerSize / 2}px`;
        alivePlayers[i].element.style.left = `${Math.cos(1.5 * Math.PI - deltaAngle * i) * tableSize / 2 + tableSize / 2 - playerSize / 2}px`;
    }
}

function playerClickListener(e) {
    let number = e.target.id.split("-");
    number = number[number.length - 1];
    for (let i of players) {
        if (i.number == number) {
            i.element.classList.add("selected-player");
            i.element.classList.remove("unselected-player");
        } else {
            i.element.classList.remove("selected-player");
            i.element.classList.add("unselected-player");
        }
    }
    redrawTable();
}

function init() {
    players = Main.players;
    tableDiv = Main.tableDiv;
    for (let i = 0; i < players.length; i++) {
        players[i].element.addEventListener("click", playerClickListener);
    }
    redrawTable();
}

let Table = {
    init: init,
}

export default Table;