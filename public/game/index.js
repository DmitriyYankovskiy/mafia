let tableDiv;
let players = [];

function Player(number) {
    this.number = number;
    this.state = "alive";
    this.element = document.getElementById(`player-${number}`);
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
    if (players.length == 0) return;
    let playerSize = window.getComputedStyle(players[0].element).width.split("p")[0];
    let tableSize = window.getComputedStyle(tableDiv).width.split("p")[0];
    //console.log(playerSize);
    let deltaAngle = Math.PI * 2 / aliveCount;
    for (let i = 0; i < aliveCount; i++) {
        alivePlayers[i].element.style.top = `${-Math.sin(1.5 * Math.PI - deltaAngle * i) * tableSize / 2 + tableSize / 2 - playerSize / 2}px`;
        alivePlayers[i].element.style.left = `${Math.cos(1.5 * Math.PI - deltaAngle * i) * tableSize / 2 + tableSize / 2 - playerSize / 2}px`;
    }
}

window.onload = function() {
    tableDiv = document.getElementById("round-table");
    for (let i = 1; i <= tableDiv.childElementCount; i++) {
        players.push(new Player(i));
    }
    redrawTable();
}