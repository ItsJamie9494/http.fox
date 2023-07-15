const ThemeRailcasts = document.getElementById("railscasts");
const ThemeTomorrow = document.getElementById("tomorrow");
const toggle = document.getElementById("dark-mode-toggle");
const button = document.querySelector("#menu-button");
const menu = document.querySelector("#menu");
const MastodonShareButton = document.getElementById("mastodon_share_button");

document.addEventListener("DOMContentLoaded", () => {
	const copyright = document.getElementById("copyright");
	if (copyright != null) {
		const year = new Date().getFullYear();
		copyright.innerText = `${year} Luna Dragon`;
	}
});

function ShareOnMastodon(): void {
	const contentelm = document.getElementById(
		"content",
	) as HTMLTextAreaElement | null;
	const InstanceUrlform = document.getElementById(
		"instance_url",
	) as HTMLInputElement | null;
	if (contentelm != null && InstanceUrlform != null) {
		const content = encodeURIComponent(contentelm.value);

		let url = InstanceUrlform.value;
		if (url === "") {
			url = "https://hachyderm.io";
		}

		const masto_url = `${url}/share?text=${content}`;

		window.open(masto_url, "_blank");
	}
}

MastodonShareButton?.addEventListener("click", () => {
	ShareOnMastodon();
});

function setTheme(mode: string): void {
	if (mode === "dark") {
		document.documentElement.classList.add("dark");
		if (toggle != null) {
			toggle.innerHTML =
				'<i id="darkmode_icon" class="fa-solid fa-sun" aria-label="Switch between dark and light mode" aria-hidden="true"></i>';
		}
		if (ThemeRailcasts != null && ThemeTomorrow != null) {
			ThemeRailcasts.removeAttribute("disabled");
			ThemeTomorrow.setAttribute("disabled", "disabled");
		}
	} else if (mode === "light") {
		document.documentElement.classList.remove("dark");
		if (toggle != null) {
			toggle.innerHTML =
				'<i id="darkmode_icon" class="fa-solid fa-moon" aria-label="Switch between dark and light mode" aria-hidden="true"></i>';
		}
		if (ThemeRailcasts != null && ThemeTomorrow != null) {
			ThemeTomorrow.removeAttribute("disabled");
			ThemeRailcasts.setAttribute("disabled", "disabled");
		}
	}
	localStorage.setItem("theme", mode);
}

if (button != null) {
	button.addEventListener("click", () => {
		if (menu != null) {
			menu.classList.toggle("hidden");
		}
	});
}

if (toggle != null) {
	if (
		localStorage.theme === "light" ||
		(!("theme" in localStorage) &&
			window.matchMedia("(prefers-color-scheme: light)").matches)
	) {
		document.documentElement.classList.remove("dark");
		toggle.innerHTML =
			'<i id="darkmode_icon" class="fa-solid fa-moon" aria-label="Switch between dark and light mode" aria-hidden="true"></i>';
		if (ThemeRailcasts != null && ThemeTomorrow != null) {
			console.log("theme switch");
			ThemeTomorrow.removeAttribute("disabled");
			ThemeRailcasts.setAttribute("disabled", "disabled");
		}
	} else {
		document.documentElement.classList.add("dark");
		toggle.innerHTML =
			'<i id="darkmode_icon" class="fa-solid fa-sun" aria-label="Switch between dark and light mode" aria-hidden="true"></i>';
		if (ThemeRailcasts != null && ThemeTomorrow != null) {
			ThemeRailcasts.removeAttribute("disabled");
			ThemeTomorrow.setAttribute("disabled", "disabled");
		}
	}

	toggle.addEventListener("click", () => {
		console.log("click");
		const icon = document.getElementById("darkmode_icon");
		if (icon != null) {
			if (
				icon.classList.contains("fa-moon") ||
				localStorage.getItem("theme") === "light"
			) {
				setTheme("dark");
			} else if (
				icon.classList.contains("fa-sun") ||
				localStorage.getItem("theme") === "dark"
			) {
				setTheme("light");
			}
		}
	});
}
