
const copperAudioSourceHolder = new ObjectHolder();

class CopperAudioSource {

    constructor() {
        this.pitch = 1;
        this.looping = false;
        this.playing = false;
        this.sourceNode = null;
        this.gainNode = copperAudioContext.createGain();
        this.pannerNode = copperAudioContext.createPanner();
        this.pannerNode.panningModel = "HRTF";
        this.pannerNode.distanceModel = "exponential";
        this.gainNode.connect(this.pannerNode);
        this.pannerNode.connect(copperAudioContext.destination);
    }

}

function copperCreateAudioSource() {
    return copperAudioSourceHolder.add(new CopperAudioSource());
}


function copperAudioSourcePlay(pointer, audioPointer) {
    const source = copperAudioSourceHolder.get(pointer);
    const audio = copperAudioHolder.get(audioPointer);
    if(!audio.loaded) return;
    source.sourceNode = copperAudioContext.createBufferSource();
    source.sourceNode.buffer = audio.audio;
    source.sourceNode.playbackRate.value = source.pitch;
    source.sourceNode.loop = source.looping;
    source.sourceNode.onended = () => {
        source.playing = source.looping;
    };
    source.sourceNode.connect(source.gainNode);
    source.sourceNode.start();
    source.playing = true;
}

function copperAudioSourceStop(pointer) {
    const source = copperAudioSourceHolder.get(pointer);
    if(!source.playing) return;
    source.sourceNode.stop();
    source.playing = false;
}

function copperAudioSourceRepeat(pointer, repeat) {
    const source = copperAudioSourceHolder.get(pointer);
    source.looping = repeat;
    if(!source.playing) return;
    source.sourceNode.loop = repeat;
}

function copperAudioSourcePlaying(pointer) {
    return copperAudioSourceHolder.get(pointer).playing;
}

function copperAudioSourceSetPosition(pointer, x, y, z) {
    const source = copperAudioSourceHolder.get(pointer);
    source.pannerNode.positionX.value = x;
    source.pannerNode.positionY.value = y;
    source.pannerNode.positionZ.value = z;
}

function copperAudioSourceSetVolume(pointer, volume) {
    copperAudioSourceHolder.get(pointer).gainNode.gain.value = volume;
}

function copperAudioSourceSetPitch(pointer, pitch) {
    const source = copperAudioSourceHolder.get(pointer);
    source.pitch = pitch;
    if(!source.playing) return;
    source.sourceNode.playbackRate.value = pitch;
}


function copperDropAudioSource(pointer) {
    copperAudioSourceHolder.drop(pointer);
}