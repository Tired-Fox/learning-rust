'use strict';
const autoprefixer = require('autoprefixer');
const cssnano = require('cssnano');
const del = require('del');
const eslint = require('gulp-eslint');
const fs = require('fs');
const path = require('path');
const postcss = require('gulp-postcss');
const sass = require('gulp-dart-sass');
const stylelint = require('gulp-stylelint');
const tildeImporter = require('node-sass-tilde-importer');
const yargs = require('yargs');
const { LintUtil: { EslintBuilder, StylelintBuilder }, GulpUtil } = require('@mcom/dev-utils');
const { series, parallel, src, dest, watch } = require('gulp');
const tar = require('gulp-tar');
const gzip = require('gulp-gzip');

process.env.APP_VERSION = GulpUtil.checkPomForProjectVersion();

const APP_NAME = 'admin-web';

const GENERATE_REPORT = yargs.argv.generateReports;
const REPORT_DIRECTORY = path.join(__dirname, '/target/reports');
const TARGET_DIRECTORY = path.join(__dirname, 'target/static');

const IS_PROD = process.env.NODE_ENV === 'production';
process.env.DEPLOY_DIRECTORY = path.join(TARGET_DIRECTORY, IS_PROD ? 'p' : 'd', APP_NAME, process.env.APP_VERSION);

const WEBPACK = require('./webpack.config');

console.log(`Building Project Version: ${process.env.APP_VERSION}`);
console.log(`Building for Environment: ${process.env.NODE_ENV}`);

function clean () {
  return del(TARGET_DIRECTORY);
}

function generateLintFiles (done) {
  // create the .stylelintrc.js file
  new StylelintBuilder(__dirname)
    .generate();

  // create the .eslintrc.js file
  new EslintBuilder(__dirname)
    .generate();

  done();
}

// Lint JS files
function lintJs() {
  let report;
  if (GENERATE_REPORT) {
    GulpUtil.createPath(REPORT_DIRECTORY);
    report = fs.createWriteStream(`${REPORT_DIRECTORY}/eslint-report.json`);
  }

  return src([`${__dirname}/src/js/**/*`, `${__dirname}/src/components/**/*`])
    .pipe(eslint())
    .pipe(eslint.format('stylish', process.stdout))
    .pipe(eslint.format(GENERATE_REPORT ? 'json' : 'stylish', report))
    .pipe(eslint.failAfterError());
}

function lintCss () {
  const reporters = [{ formatter: 'string', console: true }];
  if (GENERATE_REPORT) {
    reporters.push({ formatter: 'json', save: 'stylelint-report.json' });
  }

  return src('src/scss/**/*.scss')
    .pipe(stylelint({
      reportOutputDir: `${REPORT_DIRECTORY}`,
      reporters: reporters
    }));
}

function includeImages () {
  return src('src/images/**/*')
    .pipe(dest(path.join(process.env.DEPLOY_DIRECTORY, 'images')));
}

// Run JS Unit Tests
const testJs = GulpUtil.testJsFunc(path.join(__dirname, 'src'), GENERATE_REPORT, REPORT_DIRECTORY);

// Build files
function buildJs(done) {
  GulpUtil.buildWebpack(WEBPACK, done);
}

// Build CSS files
function buildCss () {
  return src(path.join(__dirname, 'src/scss/**/*.scss'))
    .pipe(sass.sync({ importer: tildeImporter }).on('error', sass.logError))
    .pipe(postcss([autoprefixer({ grid: true }), cssnano()]))
    .pipe(dest(path.join(process.env.DEPLOY_DIRECTORY, 'css')));
}

// Deploy assets to location
const deployAssets = series(
  GulpUtil.deployFunc(TARGET_DIRECTORY, true, 'admin-web', process.env.APP_VERSION),
  GulpUtil.deployFunc(TARGET_DIRECTORY, false, 'admin-web', process.env.APP_VERSION)
);

// Package static assets
function createArtifact() {
  return src(`${TARGET_DIRECTORY}/**/*`)
    .pipe(tar('admin-web-static-assets.tar'))
    .pipe(gzip({ append: false }))
    .pipe(dest(path.join(__dirname, 'target')));
}

function watchJS() {
  watch(
    [
      `${__dirname}/src/js/**/*`,
      `${__dirname}/src/components/**/*`
    ],
    {usePolling: true},
    series(
      lintJs,
      buildJs,
      deployAssets
    )
  );
}

function watchCSS() {
  watch(
    [
      `${__dirname}/src/scss/**/*`
    ],
    {usePolling: true},
    series(
      lintCss,
      buildCss,
      deployAssets
    )
  );
}

exports.lint = series(generateLintFiles, parallel(lintJs, lintCss));
exports.test = parallel(testJs);
exports.build = series(
  generateLintFiles,
  parallel(lintJs, lintCss),
  testJs,
  series(buildJs, buildCss, includeImages),
  createArtifact
);
exports.deploy = deployAssets;
exports.local = series(exports.build, exports.deploy);
exports.watch = series(generateLintFiles, parallel(watchJS, watchCSS));
exports.analyze = buildJs;
exports.clean = clean;
