/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        office: {
          bg: '#0d1117',
          floor: '#161b22',
          wall: '#1c2128',
          desk: '#21262d',
          active: '#238636',
          dormant: '#30363d',
          gmo: '#1f3a6e',
          etsy: '#4f2d1e',
          amazon: '#1a3a2a',
          ebay: '#1a1f3a',
          tiktok: '#2d1a3a',
          instagram: '#3a1a2d',
          website: '#1a2a3a',
        },
        agent: {
          idle: '#58a6ff',
          working: '#3fb950',
          blocked: '#f85149',
          complete: '#a5d6ff',
        },
      },
      animation: {
        'pulse-glow': 'pulse-glow 2s ease-in-out infinite',
        'float': 'float 3s ease-in-out infinite',
        'typing': 'typing 1s steps(3) infinite',
        'slide-in': 'slide-in 0.4s ease-out',
        'fade-in': 'fade-in 0.3s ease-out',
        'desk-appear': 'desk-appear 0.5s cubic-bezier(0.175, 0.885, 0.32, 1.275)',
      },
      keyframes: {
        'pulse-glow': {
          '0%, 100%': { boxShadow: '0 0 5px currentColor', opacity: '0.8' },
          '50%': { boxShadow: '0 0 20px currentColor, 0 0 40px currentColor', opacity: '1' },
        },
        'float': {
          '0%, 100%': { transform: 'translateY(0px)' },
          '50%': { transform: 'translateY(-4px)' },
        },
        'typing': {
          '0%': { content: "'...'" },
          '33%': { content: "'..'" },
          '66%': { content: "'.'" },
        },
        'slide-in': {
          from: { transform: 'translateX(100%)', opacity: '0' },
          to: { transform: 'translateX(0)', opacity: '1' },
        },
        'fade-in': {
          from: { opacity: '0', transform: 'translateY(10px)' },
          to: { opacity: '1', transform: 'translateY(0)' },
        },
        'desk-appear': {
          from: { transform: 'scale(0) rotate(-10deg)', opacity: '0' },
          to: { transform: 'scale(1) rotate(0deg)', opacity: '1' },
        },
      },
    },
  },
  plugins: [],
}
