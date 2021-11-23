;(function () {
    Array.prototype.after = function (index) {
        return this.slice(index + 1)
    }

    Array.prototype.unique = function () {
        return [...new Set(this)]
    }

    String.prototype.toNumber = function () {
        return Number.parseInt(this)
    }
})()