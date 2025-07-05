import './App.css';

import { useState } from 'react';

const color_per_grade = {
  "president": {
    "text": {
      
    }
  }
}

function UserIcon({user})
{
  return (
    <div className="has-tooltip">
      <span className='tooltip bg-red-200 text-red-800'>dderny</span>
      <img className="user-ring ring-bartender" src={user.img} alt="" />
    </div>
  );
}

function UserTag({user})
{
  return (
    <div className="tag tag-red">
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

  const user_icons = users.map(user => {
    return (<UserIcon user={user} />);
  });
  const user_tags = users.map(user => {
    return (<UserTag user={user} />);
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

        <button className="button">Shifter</button>
      </div>
    </div>
  );
}

function DayShift()
{
  let users = [
    {
      username: "ticasali",
      grade: "bartender",
      img: "https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
    },
    {
      username: "dderny",
      grade: "novice",
      img: "https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
    },
    {
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
      
      <Shift id={5} time={1} users={users}/>
      <Shift id={5} time={1} users={users}/>
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
