
const copperKeyboardHolder = new ObjectHolder();

class CopperKeyboard {

    keyFromCode(code) {
        if(code === "ControlLeft" || code === "ControlRight") return 0;
        if(code === "ShiftLeft" || code === "ShiftRight") return 1;
        if(code === "Space") return 2;
        if(code === "Backspace") return 3;
        if(code === "Enter") return 4;
        if(code === "AltLeft" || code === "AltRight") return 5;
        if(code === "KeyA") return 6;
        if(code === "KeyB") return 7;
        if(code === "KeyC") return 8;
        if(code === "KeyD") return 9;
        if(code === "KeyE") return 10;
        if(code === "KeyF") return 11;
        if(code === "KeyG") return 12;
        if(code === "KeyH") return 13;
        if(code === "KeyI") return 14;
        if(code === "KeyJ") return 15;
        if(code === "KeyK") return 16;
        if(code === "KeyL") return 17;
        if(code === "KeyM") return 18;
        if(code === "KeyN") return 19;
        if(code === "KeyO") return 20;
        if(code === "KeyP") return 21;
        if(code === "KeyQ") return 22;
        if(code === "KeyR") return 23;
        if(code === "KeyS") return 24;
        if(code === "KeyT") return 25;
        if(code === "KeyU") return 26;
        if(code === "KeyV") return 27;
        if(code === "KeyW") return 28;
        if(code === "KeyX") return 29;
        if(code === "KeyY") return 30;
        if(code === "KeyZ") return 31;
        if(code === "ArrowUp") return 32;
        if(code === "ArrowDown") return 33;
        if(code === "ArrowLeft") return 34;
        if(code === "ArrowRight") return 35;
        if(code === "Digit0") return 36;
        if(code === "Digit1") return 37;
        if(code === "Digit2") return 38;
        if(code === "Digit3") return 39;
        if(code === "Digit4") return 40;
        if(code === "Digit5") return 41;
        if(code === "Digit6") return 42;
        if(code === "Digit7") return 43;
        if(code === "Digit8") return 44;
        if(code === "Digit9") return 45;
        if(code === "F1") return 46;
        if(code === "F2") return 47;
        if(code === "F3") return 48;
        if(code === "F4") return 49;
        if(code === "F5") return 50;
        if(code === "F6") return 51;
        if(code === "F7") return 52;
        if(code === "F8") return 53;
        if(code === "F9") return 54;
        if(code === "F10") return 55;
        if(code === "F11") return 56;
        if(code === "F12") return 57;
        if(code === "Escape") return 58;
        return 59;
    }

    constructor(elementId) {
        const element = (elementId.length === 0)? document : document.getElementById(elementId);
        this.keys = new Array(60).fill(false);
        element.addEventListener("keydown", (e) => {
            if(typeof e !== "object") return;
            this.keys[this.keyFromCode(e.code)] = true;
        });
        element.addEventListener("keyup", (e) => {
            if(typeof e !== "object") return;
            this.keys[this.keyFromCode(e.code)] = false;
        });
    }

}

function copperCreateKeyboard(elementId) {
    return copperKeyboardHolder.add(new CopperKeyboard(elementId));
}


function copperKeyboardPressing(pointer, key) {
    return copperKeyboardHolder.get(pointer).keys[key];
}


function copperDropKeyboard(pointer) {
    copperKeyboardHolder.drop(pointer);
}