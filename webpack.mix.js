let mix = require('laravel-mix');

require('laravel-mix-svelte');

mix.js('assets/js/app.js', 'static/').svelte();
