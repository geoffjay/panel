export namespace main {
	
	export class Project {
	    path: string;
	    description: string;
	    modifiedTime: string;
	
	    static createFrom(source: any = {}) {
	        return new Project(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.path = source["path"];
	        this.description = source["description"];
	        this.modifiedTime = source["modifiedTime"];
	    }
	}

}

