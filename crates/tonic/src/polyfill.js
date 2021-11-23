;(function () {
    Array.prototype.after = function (index) {
        return this.slice(index + 1)
    }

    String.prototype.toNumber = function () {
        return Number.parseInt(this)
    }
})()