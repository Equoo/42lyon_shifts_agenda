import './App.css';

import { useState } from 'react';

const curday = new Date();
// check authed
const logged_in = () => {
  const token = localStorage.getItem("token");
  return token !== null && token !== undefined && token !== "";
} 
let me = null;

function api_request(route, method, data) {
	let body = null;
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
	}).then(response => {
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    return response.json();
  })
	.catch(error => {
		console.error("Error:", error);
		throw error;
	});
}

if (!logged_in()) {
  const request = {
    "username": "dderny",
    "password": "password123"
  }

  api_request("auth/login", "POST", request)
    .then(data => {
      console.log("Logged in successfully");
    })
    .catch(error => console.error("Login error:", error));
}

api_request("auth/me", "GET", {})
  .then(data => {
    me = data;
    console.log("Me:", me);
  })
  .catch(error => console.error(error));

const color_per_grade = {
  "president": {
    "text": "text-red-800",
    "bg": "bg-red-200",
    "ring": "ring-red-700"
  },
  "bartender": {
    "text": "text-red-800",
    "bg": "bg-red-200",
    "ring": "ring-red-700"
  },
  "novice": {
    "text": "text-gray-800",
    "bg": "bg-gray-100",
    "ring": "ring-gray-100"
  }
}

function UserIcon({user})
{
  const colors = color_per_grade[user.grade];
  
  return (
    <div className="has-tooltip">
      <span className={`tooltip ${colors.bg} ${colors.text}`}>{user.username}</span>
      <img className={`user-ring ${colors.ring}`} src={user.img} alt="" />
    </div>
  );
}

function UserTag({user})
{
  const colors = color_per_grade[user.grade];

  return (
    <div className={`tag ${colors.bg} ${colors.text}`}>
      <img src={user.img} alt="" />
      <span>{user.username}</span>
      <span className="material-symbols-outlined">close_small</span>
    </div>
  );
}

function Shift({id, time, users})
{
  const [isopen, setOpen] = useState(false);

  function toggleShiftBody(shift) {
    setOpen(!isopen);
  }
  function isInShift() {
    users.forEach(user => {
      if (user.id === id)
        return true;
    });
    return false;
  }

  function joinShift() {
    
  }

  function leftShift() {
    
  }

  const user_icons = users.map(user => {
    return (<UserIcon key={user.id} user={user} />);
  });
  const user_tags = users.map(user => {
    return (<UserTag key={user.id} user={user} />);
  });

  return (
    <div className="shift">
      <div className="header" onClick={toggleShiftBody}>
        <div className="flex">
          <span className="material-symbols-outlined">{(time && "clear_day") || "moon_stars"}</span>
          <div className="font-medium ml-2 text-gray-600">{(time && "Midi") || "Soir"}</div>
        </div>
        <div className="flex -space-x-1 overflow-visible">
          {user_icons}
        </div>
      </div>
      <div className={"body " + (isopen && "open")}>
        <div className="flex flex-wrap gap-1">
          {user_tags}
        </div>

        <button className="button" onClick={(isInShift() && leftShift) || joinShift}>
          {(isInShift() && "Quitter") || "Shifter"}
        </button>
      </div>
    </div>
  );
}

function DayShift()
{
  let users = [
    {
      id: 1,
      username: "ticasali",
      grade: "bartender",
      img: "https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
    },
    {
      id: 2,
      username: "dderny",
      grade: "novice",
      img: "https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
    },
    {
      id: 3,
      username: "tdaclin",
      grade: "novice",
      img: "https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
    }
  ];
  
  return (
    <div className="day-shift">
      <div className="day-shift-header">
        <h1 className="text-2xl">Lundi</h1>
        <span>30 Juin</span>
      </div>
      
      <Shift id={2} time={1} users={users}/>
      <Shift id={6} time={0} users={users}/>
    </div>
  );
}

function App() {
  return (
    <div className="container mx-auto max-w-sm">
			<h1 className="text-xl mb-2">Semaine du 30 Juin - 05 Juillet</h1>
			
			<DayShift />

		</div>
  );
}

export default App;
