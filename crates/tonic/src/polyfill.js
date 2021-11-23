;(function () {
    Array.prototype.after = function (index) {
        return this.slice(index + 1)
    }

    Array.prototype.unique = function () {
        return [...new Set(this)]
    }

    Array.prototype.len = function () {
        return this.length
    }

    Number.prototype.floor = function () {
        return Math.floor(this)
    }

    Number.prototype.forEach = function (callback) {
        for (let i = 0; i < this; i++) {
            callback(i)
        }
    }

    String.prototype.toNumber = function () {
        return Number.parseInt(this)
    }

    String.prototype.before = function (needle) {
        return this.substring(0, this.indexOf(needle))
    }

    String.prototype.after = function (needle) {
        return this.substring(this.indexOf(needle) + 1)
    }

    String.prototype.contains = function (needle) {
        return this.includes(needle)
    }
})()