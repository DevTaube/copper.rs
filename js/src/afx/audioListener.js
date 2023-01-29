
function copperAudioListenerSetPosition(x, y, z) {
    const listener = copperAudioContext.listener;
    if("positionX" in listener && "positionY" in listener && "positionZ" in listener) {
        listener.positionX.value = x;
        listener.positionY.value = y;
        listener.positionZ.value = z;
    } else {
        listener.setPosition(x, y, z);
    }
}

function copperAudioListenerSetOrientation(lookX, lookY, lookZ, upX, upY, upZ) {
    const listener = copperAudioContext.listener;
    if("forwardX" in listener && "forwardY" in listener && "forwardZ" in listener && "upX" in listener && "upY" in listener && "upZ" in listener) {
        listener.forwardX.value = lookX;
        listener.forwardY.value = lookY;
        listener.forwardZ.value = lookZ;
        listener.upX.value = upX;
        listener.upY.value = upY;
        listener.upZ.value = upZ;
    } else {
        listener.setOrientation(lookX, lookY, lookZ, upX, upY, upZ);
    }
}