


let request = {
	"username": "dderny",
	//"password": "password123"
}

function api_request(route, method, data) {
	body = null;
	if (method === "GET") {
		let queryParams = new URLSearchParams(data).toString();
		route = route + "?" + queryParams
	} else {
		body = JSON.stringify(data)
	}

	return fetch("/api/" + route, {
		method: method,
		headers: {
			"Content-Type": "application/json"
		},
		body: body
	}).then(response => response.json())
	.catch(error => {
		console.error("Error:", error);
		throw error;
	});
}

api_request("users/dderny", "GET", request)
	.then(data => console.log(data))
	.catch(error => console.error(error));
