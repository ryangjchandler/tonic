import { Server } from "@std/http"
import Trouter from 'trouter'

class Request {
    params;

    constructor(params) {
        this.params = params
    }

    has(name) {
        return this.params[name] !== undefined
    }

    get(name) {
        return this.params[name]
    }
}

export class App {
    url;
    router;
    
    constructor(url) {
        this.url = url
        this.router = new Trouter
    }

    static init(url) {
        return new App(url)
    }

    get(path, handler) {
        this.router.get(path, handler)
        return this
    }

    post(path, handler) {
        this.router.post(path, handler)
        return this
    }

    run() {
        Server.init(this.url)
            .serve((method, url) => {
                println(url)
                
                let handlers = this.router.find(method, url)

                if (handlers.handlers.length < 1) {
                    return "404 Not Found."
                }

                let handler = handlers.handlers[0];
                let request = new Request(handlers.params)

                return handler(request)
            })
    }
}