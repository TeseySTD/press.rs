module.exports = {
  content: ["./index.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        'black': '#000000',
        'alabaster': '#E5E5E5',
        'dust': '#D2D2D2',
        'silver': '#A7A7A7',
      },
      backgroundImage: {
        'main-gradient': 'linear-gradient(135deg, #000000 0%, #E5E5E5 50%, #000000 100%)',
      }
    },
  },
  plugins: [],
}