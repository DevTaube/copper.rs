
const copperSurfaceHolder = new ObjectHolder();

class CopperSurface {

    _onImageLoad(img) {
        this.canvas = document.createElement("canvas");
        this.canvas.width = img.width;
        this.canvas.height = img.height;
        this.g = this.canvas.getContext("2d");
        this.g.drawImage(img, 0, 0);
        this.loaded = true;
    }

    constructor(canvas, g, loading) {
        if(loading) {
            canvas.onload = () => { this._onImageLoad(canvas); };
        }
        this.canvas = canvas;
        this.g = g;
        this.loaded = !loading;
    }

}

const copperObservedCanvasIds = [];

function copperCreateCanvasSurface(elementId) {
    const canvas = document.getElementById(elementId);
    const g = canvas.getContext("2d");
    if(!copperObservedCanvasIds.includes(elementId)) {
        copperObservedCanvasIds.push(elementId);
        canvas.width = canvas.offsetWidth;
        canvas.height = canvas.offsetHeight;
        new ResizeObserver(() => { canvas.width = canvas.offsetWidth; canvas.height = canvas.offsetHeight; }).observe(canvas);
    }
    return copperSurfaceHolder.add(new CopperSurface(canvas, g, false));
}

function copperCreateSizedSurface(width, height) {
    const canvas = document.createElement("canvas");
    canvas.width = width;
    canvas.height = height;
    return copperSurfaceHolder.add(new CopperSurface(canvas, canvas.getContext("2d"), false));
}

function copperCreateImageSurface(imageUrl) {
    const img = new Image();
    img.src = imageUrl;
    return copperSurfaceHolder.add(new CopperSurface(img, null, true));
}


function copperSurfaceLoaded(pointer) {
    return copperSurfaceHolder.get(pointer).loaded;
}

function copperSurfaceWidth(pointer) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return 0;
    return surface.canvas.width;
}

function copperSurfaceHeight(pointer) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return 0;
    return surface.canvas.height;
}

function copperSurfaceGetPixelR(pointer, x, y) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return 0;
    return surface.g.getImageData(x, y, 1, 1).data[0];
}
function copperSurfaceGetPixelG(pointer, x, y) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return 0;
    return surface.g.getImageData(x, y, 1, 1).data[1];
}
function copperSurfaceGetPixelB(pointer, x, y) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return 0;
    return surface.g.getImageData(x, y, 1, 1).data[2];
}
function copperSurfaceGetPixelA(pointer, x, y) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return 0;
    return surface.g.getImageData(x, y, 1, 1).data[3];
}

function copperSurfaceResize(pointer, width, height) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return copperCreateSizedSurface(width, height);
    const thisCanvas = surface.canvas;
    const thisG = surface.g;
    const newCanvas = document.createElement("canvas");
    newCanvas.width = width;
    newCanvas.height = height;
    const newG = newCanvas.getContext("2d");
    const imgData = thisG.getImageData(0, 0, thisCanvas.width, thisCanvas.height).data;
    let zoomX = newCanvas.width / thisCanvas.width;
    let zoomY = newCanvas.height / thisCanvas.height;
    let i = 0;
    let w = 0;
    let h = 0;
    for(let x = 0; x < thisCanvas.width; x++) {
        for(let y = 0; y < thisCanvas.height; y++) {
            i = (y * thisCanvas.width + x) * 4;
            newG.fillStyle = `rgba(${imgData[i]}, ${imgData[i+1]}, ${imgData[i+2]}, ${imgData[i+3]/255})`;
            w = Math.floor((x + 1) * zoomX) - Math.floor(x * zoomX);
            h = Math.floor((y + 1) * zoomY) - Math.floor(y * zoomY);
            newG.fillRect(Math.floor(x * zoomX), Math.floor(y * zoomY), w, h);
        }
    }
    return copperSurfaceHolder.add(new CopperSurface(newCanvas, newG, false));
}

function copperSurfaceClearMode(pointer) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return;
    surface.g.globalCompositeOperation = "destination-out";
}
function copperSurfaceDrawMode(pointer) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return;
    surface.g.globalCompositeOperation = "source-over";
}

function copperSurfaceDrawSubSurfaceRotated(pointer, srcPointer, alpha, sX, sY, sW, sH, x, y, w, h, rX, rY, rA) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return;
    const srcSurface = copperSurfaceHolder.get(srcPointer);
    if(!srcSurface.loaded) return;
    surface.g.translate(rX, rY);
    surface.g.rotate(rA);
    surface.g.translate(-rX, -rY);
    surface.g.globalAlpha = alpha / 255;
    surface.g.drawImage(srcSurface.canvas, sX, sY, sW, sH, x, y, w, h);
    surface.g.globalAlpha = 1;
    surface.g.resetTransform();
}

function copperSurfaceDrawRectRotated(pointer, r, g, b, a, x, y, w, h, rX, rY, rA) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return;
    surface.g.translate(rX, rY);
    surface.g.rotate(rA);
    surface.g.translate(-rX, -rY);
    surface.g.fillStyle = `rgba(${r}, ${g}, ${b}, ${a / 255})`;
    surface.g.fillRect(x, y, w, h);
    surface.g.resetTransform();
}

function copperSurfaceDrawOvalRotated(pointer, r, g, b, a, x, y, w, h, rX, rY, rA) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return;
    surface.g.translate(rX, rY);
    surface.g.rotate(rA);
    surface.g.translate(-rX, -rY);
    surface.g.fillStyle = `rgba(${r}, ${g}, ${b}, ${a / 255})`;
    surface.g.beginPath();
    surface.g.ellipse(x + w / 2, y + h / 2, w / 2, h / 2, 0, 2 * Math.PI, false);
    surface.g.fill();
    surface.g.resetTransform();
}

function copperSurfaceDrawLine(pointer, r, g, b, a, x1, y1, x2, y2, w) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return;
    surface.g.strokeStyle = `rgba(${r}, ${g}, ${b}, ${a / 255})`;
    surface.g.lineWidth = w;
    surface.g.beginPath();
    surface.g.moveTo(x1, y1);
    surface.g.lineTo(x2, y2);
    surface.g.stroke();
}

function copperSurfaceDrawQuadraticBezier(pointer, r, g, b, a, x1, y1, x2, y2, x3, y3, w) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return;
    surface.g.strokeStyle = `rgba(${r}, ${g}, ${b}, ${a / 255})`;
    surface.g.lineWidth = w;
    surface.g.beginPath();
    surface.g.moveTo(x1, y1);
    surface.g.quadraticCurveTo(x2, y2, x3, y3);
    surface.g.stroke();
}

function copperSurfaceDrawCubicBezier(pointer, r, g, b, a, x1, y1, x2, y2, x3, y3, x4, y4, w) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return;
    surface.g.strokeStyle = `rgba(${r}, ${g}, ${b}, ${a / 255})`;
    surface.g.lineWidth = w;
    surface.g.beginPath();
    surface.g.moveTo(x1, y1);
    surface.g.bezierCurveTo(x2, y2, x3, y3, x4, y4);
    surface.g.stroke();
}

function copperSurfaceDrawTextRotated(pointer, text, fontFamily, r, g, b, a, x, y, textAlign, h, rX, rY, rA) {
    const surface = copperSurfaceHolder.get(pointer);
    if(!surface.loaded) return;
    surface.g.translate(rX, rY);
    surface.g.rotate(rA);
    surface.g.translate(-rX, -rY);
    surface.g.font = `${h}px ${fontFamily}`;
    surface.g.fillStyle = `rgba(${r}, ${g}, ${b}, ${a / 255})`;
    surface.g.textAlign = textAlign;
    surface.g.fillText(text, x, y + h);
    surface.g.resetTransform();
}


function copperDropSurface(pointer) {
    copperSurfaceHolder.drop(pointer);
}