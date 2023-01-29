
function copperStart() {
    copperWASM.start();
    requestAnimationFrame(copperFrame);
}

let copperLastFrame = 0;
let copperCurrentDeltaTime = 0;

function copperFrame(timestamp) {
    if(copperLastFrame != 0) {
        copperCurrentDeltaTime = (timestamp - copperLastFrame) / 1000;
    }
    copperLastFrame = timestamp;
    copperWASM.frame();
    requestAnimationFrame(copperFrame);
}

function copperDeltaTime() {
    return copperCurrentDeltaTime;
}

function copperLog(text) {
    console.log(text);
}