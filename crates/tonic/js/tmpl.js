import { File } from "@std/fs"

const interpolationOpen = "{{";
const interpolationClose = "}}";
const interpolationSplitPattern = /{{|}}/;

const parse = (template) => {
	let result = /{{(.*?)}}/g.exec(template);
	const arr = [];
	let firstPos;

	while (result) {
		firstPos = result.index;
		if (firstPos !== 0) {
			arr.push(template.substring(0, firstPos));
			template = template.slice(firstPos);
		}

		arr.push(result[0]);
		template = template.slice(result[0].length);
		result = /{{(.*?)}}/g.exec(template);
	}

	if (template) arr.push(template);
	return arr;
}

const compile = (template, data) => {
    const ast = template;
    let fn = `""`;

    ast.map(t => {
        if (t.startsWith(interpolationOpen) && t.endsWith(interpolationClose)) {
            fn += ` + (__data.${t.split(interpolationSplitPattern).filter(Boolean)[0].trim()} || '')`
        } else {
            fn += ` + "${t}"`;
        }
    })

    return new Function("__data", "return " + fn);
};

// A hyper-minimal template engine for Tonic that has support for interpolation of variables.
export class TemplateEngine {
    root;

    constructor(root) {
        this.root = root
    }

    static init(root) {
        return new TemplateEngine(root)
    }

    render(path, data = {}) {
        const contents = File.read(`${this.root}/${path}`).contents()
        const ast = parse(contents)
        
        return compile(ast)(data)
    }
}