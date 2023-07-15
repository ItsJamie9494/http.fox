const ThemeRailcasts = document.getElementById("railscasts");
const ThemeTomorrow = document.getElementById("tomorrow");
const toggle = document.getElementById("dark-mode-toggle");
const button = document.querySelector("#menu-button");
const menu = document.querySelector("#menu");
const MastodonShareButton = document.getElementById("mastodon_share_button");

window.addEventListener("DOMContentLoaded", () => {
	const CopyToClipboardButton = document.getElementById("copy_clipboard_btn");
	const UsageBox = document.getElementById("usage_box");

	// Lock the text copy function
	var hasCopiedText = false;
	CopyToClipboardButton?.addEventListener("click", async (event) => {
		if (!hasCopiedText) {
			hasCopiedText = true;

			let original_child = CopyToClipboardButton.innerHTML;
			let usage_text = UsageBox?.firstChild?.textContent;

			if (usage_text) {
				await navigator.clipboard.writeText(usage_text)

				CopyToClipboardButton.innerHTML = "Copied!";
			}

			setTimeout(() => {
				CopyToClipboardButton.innerHTML = original_child;
				hasCopiedText = false;
			}, 3000);
		}

		event.stopPropagation();
	});

	const copyright = document.getElementById("copyright");
	if (copyright != null) {
		const year = new Date().getFullYear();
		copyright.innerText = `${year} Luna Dragon`;
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