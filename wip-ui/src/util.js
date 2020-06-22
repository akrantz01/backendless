export function userInfo() {
    let data = localStorage.getItem("user");
    return JSON.parse(data);
}
