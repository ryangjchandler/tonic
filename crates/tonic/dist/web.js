// js/web.js
import { Server } from "@std/http";

// node_modules/regexparam/dist/regexparam.mjs
function regexparam_default(str, loose) {
  if (str instanceof RegExp)
    return { keys: false, pattern: str };
  var c, o, tmp, ext, keys = [], pattern = "", arr = str.split("/");
  arr[0] || arr.shift();
  while (tmp = arr.shift()) {
    c = tmp[0];
    if (c === "*") {
      keys.push("wild");
      pattern += "/(.*)";
    } else if (c === ":") {
      o = tmp.indexOf("?", 1);
      ext = tmp.indexOf(".", 1);
      keys.push(tmp.substring(1, !!~o ? o : !!~ext ? ext : tmp.length));
      pattern += !!~o && !~ext ? "(?:/([^/]+?))?" : "/([^/]+?)";
      if (!!~ext)
        pattern += (!!~o ? "?" : "") + "\\" + tmp.substring(ext);
    } else {
      pattern += "/" + tmp;
    }
  }
  return {
    keys,
    pattern: new RegExp("^" + pattern + (loose ? "(?=$|/)" : "/?$"), "i")
  };
}

// node_modules/trouter/index.mjs
var Trouter = class {
  constructor() {
    this.routes = [];
    this.all = this.add.bind(this, "");
    this.get = this.add.bind(this, "GET");
    this.head = this.add.bind(this, "HEAD");
    this.patch = this.add.bind(this, "PATCH");
    this.options = this.add.bind(this, "OPTIONS");
    this.connect = this.add.bind(this, "CONNECT");
    this.delete = this.add.bind(this, "DELETE");
    this.trace = this.add.bind(this, "TRACE");
    this.post = this.add.bind(this, "POST");
    this.put = this.add.bind(this, "PUT");
  }
  use(route, ...fns) {
    let handlers = [].concat.apply([], fns);
    let { keys, pattern } = regexparam_default(route, true);
    this.routes.push({ keys, pattern, method: "", handlers });
    return this;
  }
  add(method, route, ...fns) {
    let { keys, pattern } = regexparam_default(route);
    let handlers = [].concat.apply([], fns);
    this.routes.push({ keys, pattern, method, handlers });
    return this;
  }
  find(method, url) {
    let isHEAD = method === "HEAD";
    let i = 0, j = 0, k, tmp, arr = this.routes;
    let matches = [], params = {}, handlers = [];
    for (; i < arr.length; i++) {
      tmp = arr[i];
      if (tmp.method.length === 0 || tmp.method === method || isHEAD && tmp.method === "GET") {
        if (tmp.keys === false) {
          matches = tmp.pattern.exec(url);
          if (matches === null)
            continue;
          if (matches.groups !== void 0)
            for (k in matches.groups)
              params[k] = matches.groups[k];
          tmp.handlers.length > 1 ? handlers = handlers.concat(tmp.handlers) : handlers.push(tmp.handlers[0]);
        } else if (tmp.keys.length > 0) {
          matches = tmp.pattern.exec(url);
          if (matches === null)
            continue;
          for (j = 0; j < tmp.keys.length; )
            params[tmp.keys[j]] = matches[++j];
          tmp.handlers.length > 1 ? handlers = handlers.concat(tmp.handlers) : handlers.push(tmp.handlers[0]);
        } else if (tmp.pattern.test(url)) {
          tmp.handlers.length > 1 ? handlers = handlers.concat(tmp.handlers) : handlers.push(tmp.handlers[0]);
        }
      }
    }
    return { params, handlers };
  }
};

// js/web.js
var Request = class {
  params;
  constructor(params) {
    this.params = params;
  }
  has(name) {
    return this.params[name] !== void 0;
  }
  get(name) {
    return this.params[name];
  }
};
var App = class {
  url;
  router;
  constructor(url) {
    this.url = url;
    this.router = new Trouter();
  }
  static init(url) {
    return new App(url);
  }
  get(path, handler) {
    this.router.get(path, handler);
    return this;
  }
  post(path, handler) {
    this.router.post(path, handler);
    return this;
  }
  run() {
    Server.init(this.url).serve((method, url) => {
      println(url);
      let handlers = this.router.find(method, url);
      if (handlers.handlers.length < 1) {
        return "404 Not Found.";
      }
      let handler = handlers.handlers[0];
      let request = new Request(handlers.params);
      return handler(request);
    });
  }
};
export {
  App
};
