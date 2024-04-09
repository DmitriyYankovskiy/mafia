import Main from "./main.js";

let ws;

let Socket = {};

Socket.onmessage = function(event) {
    console.log(JSON.parse(event.data));
    let data = JSON.parse(event.data);
    if ("Sunrise" in data) {
        Main.gameEvents.startSunrise();
    } else if ("Discussion" in data) {
        Main.gameEvents.startDiscussion();
    } else if ("Voting" in data) {
        Main.gameEvents.startVoting();
    } else if ("Sunset" in data) {
        Main.gameEvents.startSunset();
    } else if ("Night" in data) {
        Main.gameEvents.startNight();
    }

    if ("Accuse" in data) {
        Main.gameEvents.addAccusions(Main.players[data.Accuse.num]);
    } else if ("Vote" in data) {
        Main.gameEvents.addVoice(Main.players[data.Vote.num]);
    } else if ("Die" in data) {
        Main.gameEvents.killPlayer(Main.players[data.Die.num]);
    }
}

Socket.setOnOpen = function(event) {
    ws = new WebSocket(`ws://${location.hostname}:9999/ws`);
    ws.onopen = event;
}

Socket.init = function () {
    ws.onmessage = Socket.onmessage;
}

Socket.send = function(obj) {
    ws.send(JSON.stringify(obj));
}


Socket.accuse = function(selectedPlayer) {
    Socket.send({
        Accuse: {
            target: selectedPlayer.number
        }
    });
}

Socket.action = function(selectedPlayer) {
    Socket.send({
        Action: {
            target: selectedPlayer.number
        }
    });
}

Socket.vote = function() {
    Socket.send({Vote: {}});
}

Socket.startGame = function() {
    return new Promise(function(resolve, reject) {
        ws.send(JSON.stringify({name: (prompt("name:", "Player") || "Player")}));
        ws.onmessage = (e) => {
            console.log(JSON.parse(e.data));
            resolve(JSON.parse(e.data));
            ws.onmessage = Socket.onmessage;
        };
    });
};

Socket.nextPhase = function(data) {

};

export default Socket;