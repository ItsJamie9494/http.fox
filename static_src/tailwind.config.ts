/** @type {import('tailwindcss').Config} */
import { Config } from "tailwindcss";

module.exports = {
	darkMode: "class",
	content: ["../templates/*.html.tera", "../templates/statuses/*.html.tera", "../static/**/*.js"],
	theme: {
		extend: {
			contrast: {
				25: ".25",
			},
		},
	},
} as Config;
