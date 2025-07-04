


let request = {
	"username": "dderny",
}

fetch("/api/users", {
	headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify(request)
})
	.then(response => response.json())
	.then(data => console.log(data))
	.catch(error => console.error(error));

function toggleShiftBody(shift) {
	const body = shift.querySelector('.body');
	if (body.classList.contains('open')) {
		body.classList.remove('open');
	} else {
		body.classList.add('open');
	}
}

document.querySelectorAll('.shift .header').forEach(header => {
	header.addEventListener('click', () => {
		const shift = header.closest('.shift');
		toggleShiftBody(shift);
	});
});