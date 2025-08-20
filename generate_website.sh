#!/bin/bash

set -e # Exit immediately if a command exits with a non-zero status. 

echo "Starting website generation..."

# 1. Clean and create the web/ directory
echo "Cleaning and creating web/ directory..."
rm -rf web
mkdir -p web
mkdir -p web/src # For input.css

# 2. Create initial HTML, CSS, JS files in web/
echo "Creating initial HTML, CSS, JS files..."
cat <<EOF > web/index.html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Mystical Runic Project</title>
    <link href="./output.css" rel="stylesheet">
    <style>
        /* Custom scrollbar for a cleaner look */
        ::-webkit-scrollbar {
            width: 8px;
        }
        ::-webkit-scrollbar-track {
            background: #0a0a0a; /* Dark background */
        }
        ::-webkit-scrollbar-thumb {
            background: #00ffcc; /* Neon Green/Cyan */
            border-radius: 4px;
        }
        ::-webkit-scrollbar-thumb:hover {
            background: #00e6b8; /* Lighter neon on hover */
        }

        /* SVG Background Styling */
        .svg-background {
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            z-index: -1;
            overflow: hidden;
            filter: blur(2px); /* Subtle blur for abstract feel */
            transition: filter 0.5s ease-out;
        }
        .svg-background svg {
            width: 100%;
            height: 100%;
            display: block;
            transform-origin: center center;
            transition: transform 0.5s ease-out, opacity 0.5s ease-out;
        }
        .svg-background path, .svg-background circle, .svg-background polygon {
            transition: transform 0.3s ease-out, fill 0.3s ease-out, stroke 0.3s ease-out, opacity 0.3s ease-out;
            transform-origin: center center;
            stroke: #00ffcc; /* Accent color for strokes */
            stroke-width: 0.5;
            fill: transparent; /* Transparent fill by default */
        }
        .svg-background .fill-shape {
            fill: #1a1a1a; /* Darker fill for some shapes */
            stroke: none;
        }
        .svg-background .accent-fill {
            fill: #00ffcc; /* Accent fill for specific shapes */
            stroke: none;
        }
    </style>
</head>
<body class="bg-black text-gray-100 font-mono leading-normal tracking-wide overflow-x-hidden">
    <header class="bg-black bg-opacity-80 text-white p-4 shadow-lg fixed w-full z-50 top-0 border-b border-gray-800">
        <nav class="container mx-auto flex justify-between items-center">
            <a href="#" class="text-3xl font-extrabold text-neon-cyan hover:text-white transition duration-300">Mystical Runic</a>
            <ul class="flex space-x-6 text-lg">
                <li><a href="#home" class="hover:text-neon-cyan transition duration-300">Home</a></li>
                <li><a href="#features" class="hover:text-neon-cyan transition duration-300">Features</a></li>
                <li><a href="#docs" class="hover:text-neon-cyan transition duration-300">Documentation</a></li>
                <li><a href="#playground" class="hover:text-neon-cyan transition duration-300">Playground</a></li>
                <li><a href="#contact" class="hover:text-neon-cyan transition duration-300">Contact</a></li>
            </ul>
        </nav>
    </header>

    <section id="home" class="relative h-screen flex items-center justify-center text-white overflow-hidden">
        <div class="svg-background">
            <svg viewBox="0 0 1000 1000" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMidYMid slice">
                <!-- Sacred Geometry / Techno inspired shapes -->
                <defs>
                    <g id="triangle">
                        <polygon points="50,0 100,86.6 0,86.6" />
                    </g>
                    <g id="hexagon">
                        <polygon points="100,0 150,86.6 100,173.2 0,173.2 -50,86.6 0,0" />
                    </g>
                    <g id="circle-pattern">
                        <circle cx="0" cy="0" r="50" />
                        <circle cx="0" cy="100" r="50" />
                        <circle cx="86.6" cy="50" r="50" />
                        <circle cx="-86.6" cy="50" r="50" />
                    </g>
                </defs>

                <!-- Background grid/lines -->
                <path d="M0 0 L1000 0 L1000 1000 L0 1000 Z" fill="#0a0a0a"></path>
                <g stroke="#1a1a1a" stroke-width="0.2">
                    <line x1="0" y1="0" x2="1000" y2="1000" class="line-diag"></line>
                    <line x1="0" y1="1000" x2="1000" y2="0" class="line-diag"></line>
                    <line x1="500" y1="0" x2="500" y2="1000" class="line-vert"></line>
                    <line x1="0" y1="500" x2="1000" y2="500" class="line-horiz"></line>
                </g>

                <!-- Detailed geometric patterns -->
                <g transform="translate(200 200) scale(0.5)" class="pattern-group-1">
                    <use href="#triangle" transform="translate(0 0) rotate(0)" class="shape-item"></use>
                    <use href="#triangle" transform="translate(100 0) rotate(60)" class="shape-item"></use>
                    <use href="#triangle" transform="translate(50 86.6) rotate(120)" class="shape-item"></use>
                    <use href="#triangle" transform="translate(150 86.6) rotate(180)" class="shape-item"></use>
                </g>

                <g transform="translate(700 300) scale(0.3)" class="pattern-group-2">
                    <use href="#hexagon" transform="translate(0 0) rotate(0)" class="shape-item"></use>
                    <use href="#hexagon" transform="translate(100 0) rotate(30)" class="shape-item"></use>
                </g>

                <g transform="translate(300 700) scale(0.2)" class="pattern-group-3">
                    <use href="#circle-pattern" transform="translate(0 0)" class="shape-item"></use>
                    <use href="#circle-pattern" transform="translate(200 0)" class="shape-item"></use>
                </g>

                <!-- Dynamic elements -->
                <circle cx="500" cy="500" r="10" fill="#00ffcc" class="dynamic-dot"></circle>
                <polygon points="500,450 550,500 500,550 450,500" fill="#00ffcc" class="dynamic-star"></polygon>
            </svg>
        </div>
        <div class="absolute inset-0 bg-black opacity-70"></div>
        <div class="z-10 text-center p-8 md:p-12 bg-black bg-opacity-70 rounded-xl shadow-2xl transform transition-all duration-700 ease-in-out hover:scale-105 border border-neon-cyan">
            <h1 class="text-6xl md:text-7xl font-extrabold mb-6 text-neon-cyan drop-shadow-lg animate-fade-in-down">
                <span class="block">Unleash the Magic</span>
                <span class="block text-4xl md:text-5xl text-gray-200 mt-2">of Templating with Mystical Runic</span>
            </h1>
            <p class="text-xl md:text-2xl mb-10 max-w-3xl mx-auto leading-relaxed animate-fade-in-up">
                A <span class="font-bold text-neon-cyan">high-performance</span>, <span class="font-bold text-neon-cyan">secure</span>, and <span class="font-bold text-neon-cyan">feature-rich</span> templating engine for Rust.
                Craft dynamic content with <span class="font-bold text-neon-cyan">ease and precision</span>.
            </p>
            <a href="#features" class="inline-block bg-neon-cyan hover:bg-white hover:text-black text-black font-bold py-4 px-10 rounded-full transition duration-300 ease-in-out transform hover:scale-110 shadow-lg text-xl animate-bounce-once">
                Discover Features
            </a>
        </div>
    </section>

    <section id="features" class="py-20 bg-gray-900 text-gray-100 border-t border-gray-800">
        <div class="container mx-auto px-4">
            <h2 class="text-5xl font-bold text-center mb-16 text-neon-cyan drop-shadow-md">Key Features</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-10">
                <div class="bg-gray-800 p-8 rounded-lg shadow-xl hover:shadow-2xl transition-shadow duration-300 border border-gray-700 transform hover:-translate-y-2">
                    <h3 class="text-3xl font-semibold mb-4 text-neon-cyan">⚡ High Performance</h3>
                    <p class="text-lg leading-relaxed">Optimized for speed and efficiency, ensuring rapid template rendering even under heavy loads.</p>
                </div>
                <div class="bg-gray-800 p-8 rounded-lg shadow-xl hover:shadow-2xl transition-shadow duration-300 border border-gray-700 transform hover:-translate-y-2">
                    <h3 class="text-3xl font-semibold mb-4 text-neon-cyan">🔒 Robust Security</h3>
                    <p class="text-lg leading-relaxed">Built with security in mind, protecting your applications from common template-related vulnerabilities.</p>
                </div>
                <div class="bg-gray-800 p-8 rounded-lg shadow-xl hover:shadow-2xl transition-shadow duration-300 border border-gray-700 transform hover:-translate-y-2">
                    <h3 class="text-3xl font-semibold mb-4 text-neon-cyan">🧩 Extensible & Flexible</h3>
                    <p class="text-lg leading-relaxed">Easily extendable with custom filters, helpers, and data sources to fit any project requirement.</p>
                </div>
                <div class="bg-gray-800 p-8 rounded-lg shadow-xl hover:shadow-2xl transition-shadow duration-300 border border-gray-700 transform hover:-translate-y-2">
                    <h3 class="text-3xl font-semibold mb-4 text-neon-cyan">🧑‍💻 Developer Friendly</h3>
                    <p class="text-lg leading-relaxed">Intuitive syntax and comprehensive tooling for a smooth and productive development experience.</p>
                </div>
                <div class="bg-gray-800 p-8 rounded-lg shadow-xl hover:shadow-2xl transition-shadow duration-300 border border-gray-700 transform hover:-translate-y-2">
                    <h3 class="text-3xl font-semibold mb-4 text-neon-cyan">🌐 Cross-Platform</h3>
                    <p class="text-lg leading-relaxed">Works seamlessly across various platforms, including WebAssembly for browser-based applications.</p>
                </div>
                <div class="bg-gray-800 p-8 rounded-lg shadow-xl hover:shadow-2xl transition-shadow duration-300 border border-gray-700 transform hover:-translate-y-2">
                    <h3 class="text-3xl font-semibold mb-4 text-neon-cyan">🌳 Rich Ecosystem</h3>
                    <p class="text-lg leading-relaxed">Integrates well with popular Rust web frameworks and offers a growing collection of utilities.</p>
                </div>
            </div>
        </div>
    </section>

    <footer class="bg-black text-gray-400 p-8 text-center border-t border-gray-800">
        <p class="text-lg">&copy; 2025 Mystical Runic. All rights reserved.</p>
        <p class="text-sm mt-2">Built with <span class="text-neon-cyan">passion</span> and <span class="text-neon-cyan">Rust</span>.</p>
    </footer>

    <script src="./script.js"></script>
</body>
</html>

cat <<EOF > web/src/input.css
@tailwind base;
@tailwind components;
@tailwind utilities;

/* Custom CSS for more refined effects */
@keyframes bounce-once {
  0%, 100% { transform: translateY(0); }
  20% { transform: translateY(-6px); }
  40% { transform: translateY(0); }
  60% { transform: translateY(-3px); }
  80% { transform: translateY(0); }
}
.animate-bounce-once {
  animation: bounce-once 1.5s ease-in-out 1;
}

/* Ensure full height for parallax section */
html, body {
    height: 100%;
}


EOF

cat <<EOF > web/script.js
// SVG Animation and Interaction
document.addEventListener('DOMContentLoaded', () => {
    const svg = document.querySelector('.svg-background svg');
    const shapes = document.querySelectorAll('.svg-background path, .svg-background circle, .svg-background polygon');

    // Mouse Move Effect
    document.addEventListener('mousemove', (e) => {
        const x = (e.clientX / window.innerWidth - 0.5) * 2; // -1 to 1
        const y = (e.clientY / window.innerHeight - 0.5) * 2; // -1 to 1

        shapes.forEach((shape, index) => {
            const strength = (index % 5 + 1) * 8; // Different strengths for different shapes
            shape.style.transform = `translate(${x * strength}px, ${y * strength}px) rotate(${x * 5}deg)`;
            shape.style.opacity = 1 - Math.abs(x) * 0.2 - Math.abs(y) * 0.2; // Subtle opacity change
        });
    });

    // Scroll Effect (subtle scaling/opacity/rotation)
    document.addEventListener('scroll', () => {
        const scrollY = window.scrollY;
        const opacity = Math.max(0.1, 1 - scrollY / 1000); // Fade out on scroll
        svg.style.opacity = opacity;
        svg.style.transform = `scale(${1 + scrollY * 0.00005}) rotate(${scrollY * 0.01}deg)`;

        shapes.forEach((shape, index) => {
            const scale = 1 + scrollY * 0.00002 * (index % 2 === 0 ? 1 : -1); // Subtle scale change
            const rotate = scrollY * 0.005 * (index % 3 === 0 ? 1 : -1); // Subtle rotation
            shape.style.transform += ` scale(${scale}) rotate(${rotate}deg)`;
        });
    });

    // Click Effect (random burst/color change)
    document.addEventListener('click', (e) => {
        const clickedShape = e.target.closest('.svg-background path, .svg-background circle, .svg-background polygon');
        if (clickedShape) {
            const originalFill = clickedShape.getAttribute('fill');
            const originalStroke = clickedShape.getAttribute('stroke');

            clickedShape.style.transition = 'none'; // Disable transition for immediate effect
            clickedShape.style.fill = '#00ffcc'; // Accent color
            clickedShape.style.stroke = '#00ffcc';
            clickedShape.style.transform += ` scale(1.5) rotate(${Math.random() * 360}deg)`;
            clickedShape.style.opacity = 0.2; // Flash effect

            setTimeout(() => {
                clickedShape.style.transition = ''; // Re-enable transition
                clickedShape.style.fill = originalFill; // Revert to original fill
                clickedShape.style.stroke = originalStroke; // Revert to original stroke
                clickedShape.style.transform = clickedShape.style.transform.replace(/scale\([^)]+\)/, '').replace(/rotate\([^)]+\)/, ''); // Remove temporary scale/rotate
                clickedShape.style.opacity = 1; // Revert opacity
            }, 150);
        }
    });
});

// Simple Fade-in Animation on Scroll (already present, ensuring it works with new classes)
const faders = document.querySelectorAll('.animate-fade-in-up, .animate-fade-in-down');
const appearOptions = {
    threshold: 0.1
};

const appearOnScroll = new IntersectionObserver(function(entries, appearOnScroll) {
    entries.forEach(entry => {
        if (!entry.isIntersecting) {
            return;
        }
        entry.target.classList.add('opacity-100');
        entry.target.classList.remove('opacity-0');
        appearOnScroll.unobserve(entry.target);
    });
}, appearOptions);

faders.forEach(fader => {
    fader.classList.add('opacity-0'); // Initial state for animation
    appearOnScroll.observe(fader);
});
EOF

# 3. Initialize Node.js project and install Tailwind CSS
echo "Initializing Node.js project and installing Tailwind CSS..."
cd web
npm init -y > /dev/null # Suppress verbose output

echo "Cleaning npm cache..."
npm cache clean --force

npm install -D tailwindcss@3.4.3 postcss autoprefixer --loglevel verbose

# Check if node_modules was created
if [ ! -d "node_modules" ]; then
    echo "Error: node_modules directory not created. npm install failed." >&2
    exit 1
fi
echo "node_modules created successfully."

# List contents of .bin directory for debugging
echo "Contents of node_modules/.bin/"
ls -l node_modules/.bin/ || true # Use || true to prevent script from exiting if directory is empty or not found

# 4. Create tailwind.config.js directly
echo "Creating tailwind.config.js..."
cat <<EOF > tailwind.config.js
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./**/*.{html,js}",
  ],
  theme: {
    extend: {
      colors: {
        'neon-cyan': '#00ffcc',
      },
      keyframes: {
        'fade-in-down': {
          '0%': { opacity: '0', transform: 'translateY(-20px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
        'fade-in-up': {
          '0%': { opacity: '0', transform: 'translateY(20px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' },
        },
        'bounce-once': {
          '0%, 100%': { transform: 'translateY(0)' },
          '20%': { transform: 'translateY(-6px)' },
          '40%': { transform: 'translateY(0)' },
          '60%': { transform: 'translateY(-3px)' },
          '80%': { transform: 'translateY(0)' },
        },
      },
      animation: {
        'fade-in-down': 'fade-in-down 0.6s ease-out forwards',
        'fade-in-up': 'fade-in-up 0.6s ease-out forwards',
        'bounce-once': 'bounce-once 1.5s ease-in-out 1',
      },
    },
  },
  plugins: [],
}
EOF

# Check if tailwind.config.js was created
if [ ! -f "tailwind.config.js" ]; then
    echo "Error: tailwind.config.js not created." >&2
    exit 1
fi
echo "tailwind.config.js created successfully."

# 5. Create postcss.config.js directly
echo "Creating postcss.config.js..."
cat <<EOF > postcss.config.js
module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
EOF

# Check if postcss.config.js was created
if [ ! -f "postcss.config.js" ]; then
    echo "Error: postcss.config.js not created." >&2
    exit 1
fi
echo "postcss.config.js created successfully."

# 6. Remove build script from package.json (if it was added)
echo "Removing build script from package.json..."
node -e "
  const fs = require('fs');
  const path = require('path');
  const pkgPath = path.join(process.cwd(), 'package.json');
  const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'));
  if (pkg.scripts && pkg.scripts.build) {
    delete pkg.scripts.build;
  }
  fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2));
"

# 7. Run initial Tailwind CSS build directly using its full path
echo "Attempting to run Tailwind CSS build using direct path..."
TAILWIND_BIN="./node_modules/.bin/tailwindcss"

# Check if the tailwindcss binary exists
if [ ! -f "$TAILWIND_BIN" ]; then
    echo "Error: Tailwind CSS binary not found at $TAILWIND_BIN."
    echo "This indicates a problem with npm install or the path."
    exit 1
fi

# Capture output and exit code
TAILWIND_BUILD_OUTPUT=$($TAILWIND_BIN -i ./src/input.css -o ./output.css 2>&1)
BUILD_EXIT_CODE=$?

# Check the exit code of the tailwindcss command
if [ $BUILD_EXIT_CODE -ne 0 ]; then
    echo "Error: Tailwind CSS build command exited with code $BUILD_EXIT_CODE."
    echo "Tailwind CSS output:"
    echo "$TAILWIND_BUILD_OUTPUT"
    exit 1
fi

# Check if output.css was generated and has content
if [ ! -s "output.css" ]; then # -s checks if file exists and has size > 0
    echo "Error: output.css is empty or not generated. Tailwind CSS build failed."
    echo "Tailwind CSS output:"
    echo "$TAILWIND_BUILD_OUTPUT"
    exit 1
fi
echo "output.css generated successfully and has content."

# Check if script.js was generated and has content
if [ ! -s "script.js" ]; then # -s checks if file exists and has size > 0
    echo "Error: script.js is empty or not generated."
    exit 1
fi
echo "script.js generated successfully and has content."

# Clean up temp files (if any were used, though not in this version)
# rm -f "$TAILWIND_STDOUT_FILE" "$TAILWIND_STDERR_FILE" 

echo "Website generation complete in the 'web/' directory."
echo "You can open web/index.html in your browser to view it."
