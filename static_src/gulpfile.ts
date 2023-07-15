import { src, dest, task, parallel } from "gulp";
/*css plugins*/
import postcss from "gulp-postcss";
import autoprefixer from "autoprefixer";
import cssnano from "cssnano";
import tailwindcss from "tailwindcss";
import gulpSass from "gulp-sass";
import dartSass from "sass";
/*js plugins*/
import gulpEsbuild from "gulp-esbuild";
import { wasmLoader } from "esbuild-plugin-wasm";

const production = process.env.NODE_ENV === "production";

const sass = gulpSass(dartSass);

const plugins = [
	tailwindcss("./tailwind.config.ts"),
	autoprefixer({ cascade: true }),
	cssnano({
		preset: [
			"advanced",
			{
				discardComments: {
					removeAll: true,
				},
				discardEmpty: true,
				normalizeUrl: true,
				autoprefixer: true,
			},
		],
	}),
];

task("build-css", () => {
	return src("src/css/bundle.scss")
		.pipe(sass().on("error", sass.logError))
		.pipe(postcss(plugins))
		.pipe(dest("../static/css/"));
});

task("transpile-ts", () => {
	return src("src/js/*.ts")
		.pipe(
			gulpEsbuild({
				target: "esnext",
				format: "esm",
				bundle: true,
				minify: production,
				treeShaking: true,
				plugins: [wasmLoader({ mode: "deferred" })],
				sourcemap: "external",
			}),
		)
		.pipe(dest("../static/js"));
});

task("default", parallel("transpile-ts", "build-css"));
