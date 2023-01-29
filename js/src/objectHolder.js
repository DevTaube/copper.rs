
class ObjectHolder {

    constructor() {
        this.items = [];
        this.freeIndices = [];
    }

    add(item) {
        const itemIndex = (this.freeIndices.length > 0)? this.freeIndices.shift() : this.items.length;
        this.items[itemIndex] = item;
        return itemIndex;
    }

    get(itemIndex) {
        return this.items[itemIndex];
    }

    drop(itemIndex) {
        this.items[itemIndex] = undefined;
        this.freeIndices.push(itemIndex);
    }

}