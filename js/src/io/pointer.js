
const copperPointerHolder = new ObjectHolder();

class CopperPointer {

    onMouseMove(x, y) {
        this.isMouse = true;
        this.mouseX = x;
        this.mouseY = y;
    }

    onMouseButton(button, state) {
        this.mouseButtons[button] = state;
    }

    constructor(elementId) {
        const element = (elementId.length === 0)? document : document.getElementById(elementId);
        this.isMouse = true;
        // mouse input
        this.mouseX = 0;
        this.mouseY = 0;
        this.mouseButtons = [false, false, false];
        element.addEventListener("contextmenu", (e) => {
            e.preventDefault();
        });
        element.addEventListener("mousedown", (e) => {
            if(typeof e !== "object") return;
            this.onMouseMove(e.clientX, e.clientY);
            this.onMouseButton(e.button, true);
        });
        element.addEventListener("mouseup", (e) => {
            if(typeof e !== "object") return;
            this.onMouseMove(e.clientX, e.clientY);
            this.onMouseButton(e.button, false);
        });
        element.addEventListener("mousemove", (e) => {
            if(typeof e !== "object") return;
            this.onMouseMove(e.clientX, e.clientY);
        });
        // touch input
        this.touchPositions = [];
        const sendTouches = (e) => {
            if(typeof e !== "object") return;
            this.isMouse = false;
            e = (typeof e.originalEvent === "undefined")? e : e.originalEvent;
            this.touchPositions = e.touches;
        };
        element.addEventListener("touchstart", sendTouches);
        element.addEventListener("touchend", sendTouches);
        element.addEventListener("touchmove", sendTouches);
    }

}

function copperCreatePointer(elementId) {
    return copperPointerHolder.add(new CopperPointer(elementId));
}


function copperPointerIsMouse(pointer) {
    return copperPointerHolder.get(pointer).isMouse;
}

function copperPointerPressing(pointer, button) {
    return copperPointerHolder.get(pointer).mouseButtons[button];
}

function copperPointerMouseX(pointer) {
    return copperPointerHolder.get(pointer).mouseX;
}

function copperPointerMouseY(pointer) {
    return copperPointerHolder.get(pointer).mouseY;
}

function copperPointerTouchCount(pointer) {
    return copperPointerHolder.get(pointer).touchPositions.length;
}
function copperPointerTouchX(pointer, touchIndex) {
    return copperPointerHolder.get(pointer).touchPositions[touchIndex].pageX;
}
function copperPointerTouchY(pointer, touchIndex) {
    return copperPointerHolder.get(pointer).touchPositions[touchIndex].pageY;
}


function copperDropPointer(pointer) {
    copperCreatePointer.drop(pointer);
}