'use strict';

const autoprefixer = require('autoprefixer');
const babel = require('gulp-babel');
const buffer = require('vinyl-buffer');
const browserify = require('browserify');
const cssnano = require('cssnano');
const del = require('del');
const eslint = require('gulp-eslint');
const filter = require('gulp-filter');
const glob = require('glob').sync;
const merge = require('merge-stream');
const postcss = require('gulp-postcss');
const rename = require('gulp-rename');
const sass = require('gulp-dart-sass');
const sourceStream = require('vinyl-source-stream');
const stylelint = require('gulp-stylelint');
const uglify = require('gulp-uglify');
const yargs = require('yargs');

const fs = require('fs');
const path = require('path');
const { series, parallel, src, dest } = require('gulp');

const projectVersion = yargs.argv.projectVersion;

const srcDir = `${__dirname}/src/main`;
const contentDir = `${__dirname}/src/content`;
const destDir = `${__dirname}/target/static-assets/admin/${projectVersion}`;
const reportDir = `${__dirname}/target/reports`;
const archiveFile = 'admin-static-assets.tar';

// Directories under /src/content/js/ containing JavaScript to be compiled from ES6 syntax
const es6Dirs = ['promotion'];

process.env.NODE_ENV = /.*SNAPSHOT.*/i.test(projectVersion)
    ? 'development'
    : 'production';

console.log(`Building Project Version: ${JSON.stringify(projectVersion)}`);
console.log(`Building for Environment: ${process.env.NODE_ENV}`);

function createPath(toCreate = '') {
    toCreate.split('/')
        .reduce((prev, dir) => {
            const curr = path.join(prev, dir, path.sep);
            if (!fs.existsSync(curr)) {
                fs.mkdirSync(curr);
            }
            return curr;
        }, '');
}

function clean() {
    return del([`${destDir}`, `${reportDir}`, `${__dirname}/target/${archiveFile}`]);
}

function copyFonts() {
    return src(`${srcDir}/fonts/**`)
        .pipe(dest(`${destDir}/fonts`));
}

function copyImages() {
    return src(`${srcDir}/images/**`)
        .pipe(dest(`${destDir}/images`));
}

function copyExcelTemplate() {
    return src(`${srcDir}/exceltemplate/**`)
        .pipe(dest(`${destDir}/exceltemplate`));
}

function copyJs() {
    // only copy files that don't need to be compiled
    const es5FileFilter = ['**/*.*'];
    for (const dir of es6Dirs) {
        es5FileFilter.push(`!**/${dir}/**/*.js`);
    }

    return src(`${contentDir}/js/**`)
        .pipe(filter(es5FileFilter))
        .pipe(dest(`${destDir}/js`));
}

function copyNonCss() {
    return src(`${srcDir}/css/**`)
        .pipe(filter(['**/*.*', '!**/*.css', '!**/*.min.css']))
        .pipe(dest(`${destDir}/css`));
}

function lintScss() {
    return src(`${srcDir}/scss/**/*.scss`)
        .pipe(filter(['**/*.*', '!**/font-awesome/**/*.*']))
        .pipe(stylelint({
            reportOutputDir: `${reportDir}`,
            reporters: [
                { formatter: 'string', console: true },
                { formatter: 'json', save: 'stylelint-report.json' }
            ]
        }));
}

function compileScss() {
    return src(`${srcDir}/scss/**/*.scss`)
        .pipe(sass.sync().on('error', sass.logError))
        .pipe(dest(`${destDir}/css`));
}

function compileCss() {
    return src(`${srcDir}/css/**/*.css`)
        .pipe(postcss([
            autoprefixer({ grid: true })
        ]))
        .pipe(dest(`${destDir}/css`));
}

function uglifyCss() {
    return src(`${destDir}/css/**/*.css`)
        .pipe(filter(['**/*.*', '!**/*.min.css']))
        .pipe(postcss([
            cssnano()
        ]))
        .pipe(rename({ extname: '.min.css' }))
        .pipe(dest(`${destDir}/css`));
}

function lintJs() {
    createPath(reportDir);
    const report = fs.createWriteStream(`${reportDir}/eslint-report.json`);

    return src([`${srcDir}/js/**/*.js`, `${__dirname}/*.js`])
        .pipe(eslint())
        .pipe(eslint.format('stylish', process.stdout))
        .pipe(eslint.format('json', report))
        .pipe(eslint.failAfterError());
}

function compileJs() {
    const es6FileFilter = [];
    for (const dir of es6Dirs) {
        es6FileFilter.push(`**/${dir}/**/*.js`);
    }

    return src(`${contentDir}/js/**`)
        .pipe(filter(es6FileFilter))
        .pipe(babel())
        .pipe(dest(`${destDir}/js`));
}

function browserifyJs() {
    const targetDirs = fs.readdirSync(`${srcDir}/js`).reduce(function (targets, directory) {
        if (fs.statSync(path.join(`${srcDir}/js`, directory)).isDirectory()) {
            targets.push(directory);
        }
        return targets;
    }, []);

    const bundles = targetDirs.map((entry) => {
        return browserify({ entries: glob(`${srcDir}/js/${entry}/**/*.js`), debug: process.env.NODE_ENV === 'development' })
            .transform('babelify')
            .bundle()
            .pipe(sourceStream(`${entry}.js`))
            .pipe(buffer())
            .pipe(dest(`${destDir}/js`));
    });

    return merge(bundles);
}

function uglifyJs() {
    return src(`${destDir}/js/**/*.js`)
        .pipe(filter(['**/*.*', '!**/*.min.js']))
        .pipe(uglify({
            output: {
                comments: /(?:^!|@(?:license|preserve|cc_on))/
            }
        }))
        .pipe(rename({ extname: '.min.js' }))
        .pipe(dest(`${destDir}/js`));
}

exports.clean = clean;
exports.lint = parallel(lintScss, lintJs);
exports.build = series(
    clean,
    parallel(
        copyExcelTemplate,
        copyFonts,
        copyImages,
        copyNonCss,
        series(lintScss, compileScss, compileCss, uglifyCss),
        series(lintJs, browserifyJs, uglifyJs, compileJs, copyJs)
    )
);
