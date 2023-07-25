import { src, dest, task, parallel, series, watch } from "gulp";
/*css plugins*/
import postcss from "gulp-postcss";
import autoprefixer from "autoprefixer";
import cssnano from "cssnano";
import tailwindcss from "tailwindcss";
import gulpSass from "gulp-sass";
import dartSass from "sass";
/*js plugins*/
import gulpEsbuild from "gulp-esbuild";

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

task("watch", () => {
	watch("src/css/*.scss", series(["build-css"]));
	watch("src/js/*.ts", series(["transpile-ts"]));
});

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
				sourcemap: "external",
			}),
		)
		.pipe(dest("../static/js"));
});

task("default", parallel("transpile-ts", "build-css"));
