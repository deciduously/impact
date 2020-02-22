const purgecss = require('@fullhuman/postcss-purgecss')

module.exports = {
  syntax: 'postcss-scss',
  plugins: [
    require("autoprefixer"),
    purgecss({
      content: ['./**/*.html']
    }),
    require("cssnano")({
      preset: "default"
    })
  ]
};