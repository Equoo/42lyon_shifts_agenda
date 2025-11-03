import "./App.css";
import { useEffect, useMemo, useState } from "react";

function useAutoTheme() {
  useEffect(() => {
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    const apply = () => {
      if (mediaQuery.matches) {
        document.documentElement.classList.add("dark");
      } else {
        document.documentElement.classList.remove("dark");
      }
    };
    apply();
    mediaQuery.addEventListener("change", apply);
    return () => mediaQuery.removeEventListener("change", apply);
  }, []);
}


function ThemeToggle() {
  const [dark, setDark] = useState(
    document.documentElement.classList.contains("dark")
  );

  const toggle = () => {
    const html = document.documentElement;
    if (html.classList.contains("dark")) {
      html.classList.remove("dark");
      setDark(false);
    } else {
      html.classList.add("dark");
      setDark(true);
    }
  };

  return (
    <button
      onClick={toggle}
      className="p-2 rounded bg-gray-200 dark:bg-gray-700 hover:opacity-80 transition"
    >
      {dark ? "☀️" : "🌙"}
    </button>
  );
}

// ======================================
// API helper
// ======================================
async function api_request(route, method = "GET", data = null) {
  let url = "/api/" + route;
  const options = {
    method,
    headers: {
      "Content-Type": "application/json",
    },
    credentials: "include", // send session cookie
  };

  if (method === "GET" && data) {
    const qs = new URLSearchParams(data).toString();
    url += "?" + qs;
  } else if (data) {
    options.body = JSON.stringify(data);
  }

  const res = await fetch(url, options);
  if (res.status === 204) return null;
  if (res.status === 401) throw new Error("unauthorized");
  if (!res.ok) throw new Error(`HTTP ${res.status}`);
  return res.json();
}

// ======================================
// Helpers for dates
// ======================================
function startOfWeek(date) {
  const d = new Date(date);
  const day = d.getDay();
  const diff = day === 0 ? -6 : 1 - day;
  d.setDate(d.getDate() + diff);
  d.setHours(0, 0, 0, 0);
  return d;
}
function addDays(date, n) {
  const d = new Date(date);
  d.setDate(d.getDate() + n);
  return d;
}
function formatDate(date) {
  return date.toISOString().slice(0, 10);
}
function randomAvatar(seed) {
  return `https://api.dicebear.com/7.x/identicon/svg?seed=${seed}`;
}

// ======================================
// Colors per grade
// ======================================
const color_per_grade = {
  President: { text: "text-red-800", bg: "bg-red-200", ring: "ring-red-700" },
  Bartender: { text: "text-orange-800", bg: "bg-orange-200", ring: "ring-orange-700" },
  Partner: { text: "text-blue-800", bg: "bg-blue-200", ring: "ring-blue-700" },
  Novice: { text: "text-gray-800", bg: "bg-gray-100", ring: "ring-gray-300" },
  Interested: { text: "text-green-800", bg: "bg-green-100", ring: "ring-green-300" },
  Unknown: { text: "text-gray-600", bg: "bg-gray-200", ring: "ring-gray-300" },
};

// ======================================
// Toasts
// ======================================
function Toasts({ toasts, dismiss }) {
  return (
    <div className="fixed top-3 right-3 flex flex-col gap-2 z-50">
      {toasts.map((t) => (
        <div
		  key={t.id}
          className={`px-4 py-2 rounded shadow bg-white border-l-4 ${
            t.type === "success" ? "border-green-400" : "border-red-400"
          }`}
        >
          <div className="flex items-center justify-between gap-2">
            <span>{t.message}</span>
            <button onClick={() => dismiss(t.id)}>×</button>
          </div>
        </div>
      ))}
    </div>
  );
}

// ======================================
// UI atoms
// ======================================
function UserIcon({ user }) {
  const colors = color_per_grade[user.grade] || color_per_grade.Unknown;
  return (
    <div className="has-tooltip">
      <span className={`tooltip ${colors.bg} ${colors.text}`}>{user.login}</span>
      <img
        className={`user-ring ${colors.ring}`}
        src={user.img_url || randomAvatar(user.login)}
        alt={user.login}
      />
    </div>
  );
}

function UserTag({ user }) {
  const colors = color_per_grade[user.grade] || color_per_grade.Unknown;
  return (
    <div className={`tag ${colors.bg} ${colors.text}`}>
      <img src={user.img_url || randomAvatar(user.login)} alt={user.login} />
      <span>{user.login}</span>
    </div>
  );
}

// ======================================
// Shift card
// ======================================

function ShiftCard({ shift, currentUser, onUpdate, pushToast }) {
  const [open, setOpen] = useState(false);

  const users = shift.users || [];
  const minUsers = shift.min_users ?? 2;
  const hasCurrent = currentUser
    ? users.some((u) => u.login === currentUser.login)
    : false;
  const hasBartender = users.some(
    (u) => u.grade === "Bartender" || u.grade === "President"
  );
  const isCompleted = users.length >= minUsers && hasBartender;
  const missing = Math.max(0, minUsers - users.length);

  const colorClass = isCompleted
    ? "shift-completed"
    : missing === 1
    ? "shift-almost"
    : "shift-empty";

  // 🕐 Check if shift date is before today
  const todayStr = formatDate(new Date());
  const isPast = new Date(shift.date) < new Date(todayStr);

  async function handleRegister() {
    if (isPast) return;
    try {
      const updated = await api_request("shifts/register", "GET", {
        date: shift.date,
        slot: shift.slot,
      });
      onUpdate(updated);
      pushToast({
        type: "success",
        message: `Inscrit au shift ${shift.date} (${shift.slot})`,
      });
    } catch {
      pushToast({ type: "error", message: `Erreur d'inscription` });
    }
  }

  async function handleDeregister() {
    if (isPast) return;
    try {
      const updated = await api_request("shifts/deregister", "GET", {
        date: shift.date,
        slot: shift.slot,
      });
      onUpdate(updated);
      pushToast({
        type: "success",
        message: `Désinscrit du shift ${shift.date} (${shift.slot})`,
      });
    } catch {
      pushToast({ type: "error", message: `Erreur de désinscription` });
    }
  }

  return (
    <div className={`shift ${colorClass}`}>
      <div className="header" onClick={() => setOpen((o) => !o)}>
        <div className="flex items-center gap-2">
          <span className="material-symbols-outlined">
            {shift.slot === "day" ? "clear_day" : "moon_stars"}
          </span>
          <div className="font-medium">
            {shift.slot === "day" ? "Midi" : "Soir"} • {shift.date}
            {isCompleted ? (
			  <span className="ml-2 text-green-700 text-sm">(Complété)</span>
			) : hasBartender ? (
			  <span className="ml-2 text-orange-700 text-sm">{missing} manquant(s)</span>
			) : (
			  <div className="ml-2 text-xs text-red-600 dark:text-red-400 mt-1">
				1 Bartender minimum
			  </div>
			)}
          </div>
        </div>
        <div className="flex -space-x-1 overflow-visible">
          {users.map((u) => (
            <UserIcon key={u.login} user={u} />
          ))}
        </div>
      </div>

      <div className={`body ${open ? "open" : ""}`}>
        <div className="flex flex-wrap gap-1 mb-2">
          {users.map((u) => (
            <UserTag key={u.login} user={u} />
          ))}
          {!users.length && <span className="text-sm text-gray-400">Vide</span>}
        </div>

        {/* 🚫 Block registration for past shifts */}
        <button
          className={`button ${
            isPast ? "opacity-50 cursor-not-allowed" : ""
          }`}
          disabled={isPast}
          title={isPast ? "Impossible de s'inscrire à un shift passé" : ""}
          onClick={hasCurrent ? handleDeregister : handleRegister}
        >
          {isPast
            ? "Shift passé"
            : hasCurrent
            ? "Se désinscrire"
            : "S'inscrire"}
        </button>
      </div>
    </div>
  );
}


// ======================================
// Day column
// ======================================
function DayColumn({ dateStr, shiftsForDay, currentUser, onShiftUpdate, pushToast }) {
  const todayStr = formatDate(new Date());
  const isToday = dateStr === todayStr;

  const displayDate = new Date(dateStr).toLocaleDateString("fr-FR", {
    weekday: "long",
    day: "numeric",
    month: "short",
  });

  return (
    <div className={`day-shift ${isToday ? "today-highlight" : ""}`}>
      <div className="day-shift-header flex items-center justify-between">
        <h2 className="text-lg capitalize">{displayDate}</h2>
        {isToday && <span className="text-blue-600 font-semibold text-sm">Aujourd’hui</span>}
      </div>

      {["day", "night"].map((slot) => {
        const sh = shiftsForDay.find((s) => s.slot === slot);
        return (
          <ShiftCard
            key={slot}
            shift={sh}
            currentUser={currentUser}
            onUpdate={onShiftUpdate}
            pushToast={pushToast}
          />
        );
      })}
    </div>
  );
}

// ======================================
// MAIN APP
// ======================================
function App() {
  const [me, setMe] = useState(null);

  useAutoTheme();

  // 🗓️ Start from today instead of startOfWeek()
  const [weekStart, setWeekStart] = useState(() => {
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    return today;
  });

  const [weekShifts, setWeekShifts] = useState([]);
  const [activeTab, setActiveTab] = useState("all");
  const [toasts, setToasts] = useState([]); 

  function pushToast(t) {
    const id = Date.now() + Math.random();
    setToasts((prev) => [...prev, { ...t, id }]);
    setTimeout(() => setToasts((prev) => prev.filter((x) => x.id !== id)), 4000);
  }
  function dismissToast(id) {
    setToasts((prev) => prev.filter((x) => x.id !== id));
  }

	const params = new URLSearchParams(window.location.search);
  	const code = params.get('code');


  // 🚀 OAuth-based login
  useEffect(() => {
    (async () => {
      if (code) {
		try {
			await api_request("auth/42/callback", "POST", {
				code: code
			});
			const user = await api_request("auth/me");
        	setMe(user);
			window.location.href = "/";
		} catch (e) {
			console.error(e);
		}	
		return
	  }

	  try {
        const user = await api_request("auth/me");
        setMe(user);
      } catch (e) {
        if (e.message === "unauthorized") {
          // not logged in → redirect to 42 OAuth
			const url = await api_request("auth/42/login", "POST")
			window.location.href = url;
        } else {
          console.error(e);
          pushToast({ type: "error", message: "Erreur d'authentification" });
        }
      }
    })();
  }, []);

  // fetch shifts for the week
  useEffect(() => {
    if (!me) return;
    const startStr = formatDate(weekStart);
    api_request("shifts/week", "GET", { start: startStr })
      .then((data) => setWeekShifts(data || []))
      .catch(() =>
        pushToast({ type: "error", message: "Erreur lors du chargement des shifts" })
      );
  }, [weekStart, me]);

  // websocket updates
  useEffect(() => {
    if (!me) return;
    const ws = new WebSocket("ws://localhost:3000/ws");
    ws.onopen = () => {
      ws.send(JSON.stringify({ type: "subscribe", week_start: formatDate(weekStart) }));
    };
    ws.onmessage = (evt) => {
      try {
        const msg = JSON.parse(evt.data);
        if (msg.type === "shift.updated" || msg.type === "shift.created") {
          const updated = msg.shift;
          setWeekShifts((prev) => {
            const exists = prev.some(
              (s) => s.date === updated.date && s.slot === updated.slot
            );
            if (!exists) return [...prev, updated];
            return prev.map((s) =>
              s.date === updated.date && s.slot === updated.slot ? updated : s
            );
          });
        }
      } catch (e) {
        console.error("bad ws msg", e);
      }
    };
    ws.onerror = (e) => console.error("ws error", e);
    return () => ws.close();
  }, [weekStart, me]);

  // build full week
  const fullWeek = useMemo(() => {
    const days = [];
    for (let i = 0; i < 7; i++) {
      const d = addDays(weekStart, i);
      const dStr = formatDate(d);
      const shiftsThatDay = weekShifts.filter((s) => s.date === dStr);
      const ensured = ["day", "night"].map((slot) => {
        const existing = shiftsThatDay.find((s) => s.slot === slot);
        if (existing) return existing;
        return { date: dStr, slot, min_users: 2, users: [] };
      });
      days.push({ date: dStr, shifts: ensured });
    }
    return days;
  }, [weekStart, weekShifts]);

  const myWeek = useMemo(() => {
    if (!me) return fullWeek;
    return fullWeek.map((day) => {
      const filtered = day.shifts.filter((s) =>
        (s.users || []).some((u) => u.login === me.login)
      );
      return { date: day.date, shifts: filtered.length ? filtered : [] };
    });
  }, [fullWeek, me]);

  function handleShiftUpdate(updatedShift) {
    setWeekShifts((prev) => {
      const exists = prev.some(
        (s) => s.date === updatedShift.date && s.slot === updatedShift.slot
      );
      if (!exists) return [...prev, updatedShift];
      return prev.map((s) =>
        s.date === updatedShift.date && s.slot === updatedShift.slot
          ? updatedShift
          : s
      );
    });
  }

  const endOfWeek = addDays(weekStart, 6);
  const weekTitle = `${weekStart.toLocaleDateString("fr-FR")} → ${endOfWeek.toLocaleDateString(
    "fr-FR"
  )}`;

  if (window.location.href.includes("api"))
  {
	return (<p>API route</p>);
  }

  return (
    <div className="container mx-auto max-w-5xl p-4">
      <Toasts toasts={toasts} dismiss={dismissToast} />

      {me && (
        <div className="flex items-center justify-between mb-4">
          <div>
            <h1 className="text-xl font-semibold">Planning des shifts</h1>
            <p className="text-gray-500 text-sm">Du {weekTitle}</p>
          </div>
          <div className="flex items-center gap-4">
            <div className="flex items-center gap-2">
              <div
                className="w-8 h-8 rounded-full"
                style={{
                  backgroundImage: `url(${me.img_url})`,
                  backgroundSize: "cover",
                  backgroundPosition: "center",
                }}
              ></div>
              <span className="font-medium text-gray-700">{me.login}</span>
            </div>
			<ThemeToggle />
            <button
              className="button bg-red-100 text-red-700"
              onClick={() => (window.location.href = "/api/auth/logout")}
            >
              Se déconnecter
            </button>
          </div>
        </div>
      )}

      {!me && <p>Connexion à 42 en cours...</p>}

      {me && (
        <>
          <div className="mb-4 flex gap-2">
            <button
              className="button"
              onClick={() => setWeekStart((d) => addDays(d, -7))}
            >
              ‹ 7 jours précédents
            </button>
            <button
              className="button"
              onClick={() => setWeekStart(new Date())}
            >
              Aujourd’hui
            </button>
            <button
              className="button"
              onClick={() => setWeekStart((d) => addDays(d, 7))}
            >
              7 jours suivants ›
            </button>
          </div>

          <div className="grid gap-2">
            {(activeTab === "all" ? fullWeek : myWeek).map((day) => {
              if (activeTab === "mine" && day.shifts.length === 0) return null;
              return (
                <DayColumn
                  key={day.date}
                  dateStr={day.date}
                  shiftsForDay={day.shifts}
                  currentUser={me}
                  onShiftUpdate={handleShiftUpdate}
                  pushToast={pushToast}
                />
              );
            })}
          </div>
        </>
      )}
    </div>
  );
}

export default App;

