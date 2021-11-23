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