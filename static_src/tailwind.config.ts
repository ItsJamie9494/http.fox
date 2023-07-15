/** @type {import('tailwindcss').Config} */
import { Config } from "tailwindcss";

module.exports = {
	darkMode: "class",
	content: ["../templates/*.html.tera", "../static/**/*.js"],
	plugins: [require("@tailwindcss/forms")],
} as Config;
