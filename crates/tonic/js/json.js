export class JSON {
    static encode(value, indentation = 4) {
        return globalThis.JSON.stringify(value, null, indentation)
    }

    static decode(value) {
        return globalThis.JSON.parse(value)
    }
}