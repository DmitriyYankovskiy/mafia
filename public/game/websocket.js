import Main from "./main.js";

let ws;

let Socket = {};

Socket.onmessage = function(event) {
    console.log(JSON.parse(event.data));
    let data = JSON.parse(event.data);
    if ("Sunrise" == data.NextPhase) {
        Main.gameEvents.startSunrise();
    } else if ("Discussion" == data.NextPhase) {
        Main.gameEvents.startDiscussion();
    } else if ("Voting" == data.NextPhase) {
        Main.gameEvents.startVoting(data.AllVoted);
    } else if ("Sunset" == data.NextPhase) {
        Main.gameEvents.startSunset();
    } else if ("Night" == data.NextPhase) {
        Main.gameEvents.startNight();
    }
    if ("Action" in data) {
        if ("Accuse" in data.Action) {
            Main.gameEvents.addAccusion(Main.players[data.Action.Accuse.num]);
        } else if ("Vote" in data.Action) {
            Main.gameEvents.addVoice(Main.players[data.Action.Vote.num]);
        } else if ("Die" in data.Action) {
            Main.gameEvents.killPlayer(Main.players[data.Action.Die.num]);
        }
    }

    if ("WhoTell" in data) {
        Main.gameEvents.nextDiscussioner(Main.players[data.WhoTell]);
    }

    
    if ("WhomVoted" in data) {
        Main.gameEvents.votingFor(Main.players[data.WhomVoted]);
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
    Socket.send("Vote");
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