import Main from "./main.js";

let ws;

let Socket = {};

Socket.onmessage = function(event) {
    console.log(JSON.parse(event.data));
    let data = JSON.parse(event.data);
    // if ("Sunrise" == data.NextPhase) {
    //     Main.gameEvents.startSunrise();
    // } else if ("Discussion" == data.NextPhase) {
    //     Main.gameEvents.startDiscussion();
    // } else if ("Voting" == data.NextPhase) {

    //     let votedPlayers = [];
    //     for (let i of data.Votes) {
    //         votedPlayers.push(Main.players[i]);
    //     }

    //     Main.gameEvents.startVoting(votedPlayers);
    // } else if ("Sunset" == data.NextPhase) {
    //     Main.gameEvents.startSunset();
    // } else if ("Night" == data.NextPhase) {
    //     Main.gameEvents.startNight();
    // }
    // if ("" in data) {
    //     if ("Accuse" in data.Action) {
    //         Main.gameEvents.addAccusion(Main.players[data.Action.Accuse.num]);
    //     } else if ("Vote" in data.Action) {
    //         Main.gameEvents.addVoice(Main.players[data.Action.Vote.num]);
    //     } else if ("Die" in data.Action) {
    //         Main.gameEvents.killPlayer(Main.players[data.Action.Die.num]); 
    //     }
    // }
    if ("Time" in data.Game) {
        switch (data.Game.Time.phase) {
            case "Sunrise":
                Main.gameEvents.startSunrise();
                break;
            case "Discussion":
                Main.gameEvents.startDiscussion();
                break;
            case "Voting":
                let candidates = [];
                for (let i of data.Game.Time.candidates) {
                    candidates.push(Main.players[i]);
                }
                Main.gameEvents.startVoting(candidates);
                break;
            case "Sunset":
                Main.gameEvents.startSunset();
                break;
            case "Night":
                Main.gameEvents.startNight();
                break;

        } 
    }
    if ("Next" in data.Game) {
        if (Main.phase.name == "discussion") {
            console.log(data.Game.Next.num);
            Main.gameEvents.nextDiscussioner(Main.players[data.Game.Next.num]);
        } else if (Main.phase.name == "voting") {
            Main.gameEvents.votingFor(Main.players[data.Game.Next.num]);
        }
    }
    if ("Die" in data.Game) {
        Main.gameEvents.killPlayer(Main.players[data.Game.Die.num]); 
    }
    if ("Vote" in data.Game) {
        Main.gameEvents.addVoice(Main.players[data.Game.Vote.from]); 
    }
    if ("Accuse" in data.Game) {
        Main.gameEvents.addAccusion(Main.players[data.Game.Accuse.num]);
    }
    if ("Check" in data.Game) {
        Main.gameEvents.addKnowedRole(Main.players[data.Game.Check.num], data.Game.Check.res);
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
        Game: {
            Accuse: {
                target: selectedPlayer.number
            }
        }
    });
}

Socket.action = function(selectedPlayer) {
    Socket.send({
        Game: {
            Action: {
                target: selectedPlayer.number
            }
        }
    });
}

Socket.vote = function() {
    Socket.send({
        Game: "Vote"
    });
}

Socket.startGame = function() {
    return new Promise(function(resolve, reject) {
        ws.send(JSON.stringify({name: (prompt("name:", "Player") || "Player")}));
        ws.onmessage = (e) => {
            console.log(JSON.parse(e.data));
            resolve(JSON.parse(e.data).Game.Start);
            ws.onmessage = Socket.onmessage;
        };
    });
};

Socket.nextPhase = function(data) {

};

export default Socket;