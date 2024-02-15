import Main from "./main.js";

let ws = new WebSocket(`ws://${location.hostname}:9998`);

function init() {

}

let Socket = {
    init: init
}
export default Socket;