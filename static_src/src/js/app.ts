const ThemeToggle = document.getElementById("theme_toggle");
const ThemeIcon = document.getElementById("theme_icon");
const CopyToClipboardButton = document.getElementById("copy_clipboard_btn");
const UsageBox = document.getElementById("usage_box");
const CopyrightYear = document.getElementById("copyright_year");

enum Theme {
	Light = "light",
	Dark = "dark",
}

function getTheme(): Theme {
	if (
		localStorage.theme === "light" ||
		(!("theme" in localStorage) &&
			window.matchMedia("(prefers-color-scheme: light)").matches)
	) {
		return Theme.Light;
	} else {
		return Theme.Dark;
	}
}

function setTheme(): void {
	const theme = getTheme();

	if (theme === Theme.Light) {
		document.documentElement.classList.remove("dark");
		if (ThemeIcon && ThemeToggle) {
			(ThemeIcon as HTMLImageElement).alt = "Moon Icon";
			(ThemeIcon as HTMLImageElement).src = "/static/img/moon.svg";
			ThemeToggle.title = "Enable Dark Mode";
		}
	} else if (theme === Theme.Dark) {
		document.documentElement.classList.add("dark");
		if (ThemeIcon && ThemeToggle) {
			(ThemeIcon as HTMLImageElement).alt = "Sun Icon";
			(ThemeIcon as HTMLImageElement).src = "/static/img/sun.svg";
			ThemeToggle.title = "Enable Light Mode";
		}
	}
}

window.addEventListener("DOMContentLoaded", () => {
	// Lock the text copy function
	let hasCopiedText = false;
	CopyToClipboardButton?.addEventListener("click", async (event) => {
		if (!hasCopiedText) {
			hasCopiedText = true;

			const original_child = CopyToClipboardButton.innerHTML;
			const usage_text = UsageBox?.firstChild?.textContent;

			if (usage_text) {
				await navigator.clipboard.writeText(usage_text);

				CopyToClipboardButton.innerHTML = "Copied!";
			}

			setTimeout(() => {
				CopyToClipboardButton.innerHTML = original_child;
				hasCopiedText = false;
			}, 3000);
		}

		event.stopPropagation();
	});

	ThemeToggle?.addEventListener("click", (event) => {
		event.preventDefault();

		const theme = getTheme();

		localStorage.setItem(
			"theme",
			theme === Theme.Light ? Theme.Dark : Theme.Light,
		);

		setTheme();
	});

	if (CopyrightYear) {
		const year: Number = new Date().getFullYear();
		CopyrightYear.innerText = year.toString();
	}

	setTheme();
});
