const colors = require('tailwindcss/colors')

module.exports = {
  content: ['./src/**/*.scss', './src/**/*.njk', './src/**/*.md'],
  theme: {
    colors: {
      // - logo, navigation
      'theme-primary': '#4338CA',
      'theme-primary-glare': '#6366F1',
      'theme-bg': '#ffffff',
      // - inline code (code selection)
      'selection-bg': '#4338CA',
      // - button border
      'action-border': '#6366F1',
      'text-primary': colors.black,
      // - supportive text: intro, figcaption
      'text-secondary': '#4338CA',
      // - line separator (stroke)
      stroke: '#4338CA',
      transparent: 'transparent',
      white: colors.white,
    },
    fontFamily: {
      sans: ['Fira Sans'],
      serif: ['Lora'],
      mono: ['Victor Mono'],
    },
    extend: {
      flex: {
        '0-1': '0 0 7rem',
        '0-2': '0 0 12rem',
      },
    },
    variants: {},
    plugins: [],
  },
}
