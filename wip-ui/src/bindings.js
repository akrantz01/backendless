import axios from "axios";

const url = "https://api.bindings.tech";

async function request(method, path, body=null) {
    let options = {
        method: method,
        url: `${url}${path}`,
        withCredentials: true,
        responseType: "json"
    }
    if (body !== null) {
        options.headers = {"Content-Type": "application/json"};
        options.data = body;
    }

    let resp = await axios.request(options);
    return { status: resp.status, ...resp.data};
}

export class Authentication {
    static async register(email, username, password) {
        return await request("POST", "/authentication/register", { email, username, password });
    }

    static async login(email, password) {
        return await request("POST", "/authentication/login", { email, password });
    }

    static async logout() {
        return await request("GET", "/authentication/logout");
    }
}

export class Users {
    static async read() {
        return await request("GET", "/user");
    }

    static async update(email, password) {
        return await request("PUT", "/user", {email, password});
    }

    static async delete() {
        return await request("DELETE", "/user");
    }
}

export class Projects {
    static async list() {
        return await request("GET", "/projects");
    }

    static async create(name, description) {
        return await request("POST", "/projects", { name, description });
    }

    static async read(id) {
        return await request("GET", `/projects/${id}`);
    }

    static async update(id, name, description) {
        return await request("PUT", `/projects/${id}`, { name, description });
    }

    static async delete(id) {
        return await request("DELETE", `/projects/${id}`);
    }
}

export class Deployments {
    static async list(id) {
        return await request("GET", `/projects/${id}/deployments`);
    }

    static async create(id, handlers, name, routes, static_directory, version) {
        return await request("POST", `/projects/${id}/deployments`);
    }

    static async add_static(id, project_id) {
        // TODO: implement static upload
    }

    static async read(id, project_id) {
        return await request("GET", `/projects/${project_id}/deployments/${id}`);
    }

    static async delete(id, project_id) {
        return await request("DELETE", `/projects/${project_id}/deployments/${id}`);
    }
}
