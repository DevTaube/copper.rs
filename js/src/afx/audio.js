
const copperAudioContext = new (window.AudioContext || window.webkitAudioContext)();

const copperAudioHolder = new ObjectHolder();

class CopperAudio {

    constructor(audioUrl) {
        this.audio = null;
        this.loaded = false;
        fetch(audioUrl)
            .then(response => response.arrayBuffer())
            .then(responseBuffer => copperAudioContext.decodeAudioData(responseBuffer))
            .then(audioBuffer => {
                this.audio = audioBuffer;
                this.loaded = true;
            });
    }

}

function copperCreateAudio(audioUrl) {
    return copperAudioHolder.add(new CopperAudio(audioUrl));
}


function copperAudioLoaded(pointer) {
    return copperAudioHolder.get(pointer).loaded;
}


function copperDropAudio(pointer) {
    copperAudioHolder.drop(pointer);
}