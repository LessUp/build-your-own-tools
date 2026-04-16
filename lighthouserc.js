module.exports = {
  ci: {
    collect: {
      url: ['http://localhost:4173/build-your-own-tools/'],
      startServerCommand: 'npm run docs:preview',
      startServerReadyPattern: 'Network:',
      startServerReadyTimeout: 30000,
      numberOfRuns: 3,
    },
    assert: {
      preset: 'lighthouse:recommended',
      assertions: {
        'categories:performance': ['warn', { minScore: 0.8 }],
        'categories:accessibility': ['error', { minScore: 0.9 }],
        'categories:best-practices': ['warn', { minScore: 0.9 }],
        'categories:seo': ['error', { minScore: 0.9 }],
        'categories:pwa': 'off',
        
        // Performance metrics
        'first-contentful-paint': ['warn', { maxNumericValue: 2000 }],
        'largest-contentful-paint': ['warn', { maxNumericValue: 2500 }],
        'cumulative-layout-shift': ['error', { maxNumericValue: 0.1 }],
        'total-blocking-time': ['warn', { maxNumericValue: 300 }],
        
        // Relaxed for documentation sites
        'unused-javascript': 'off',
        'uses-responsive-images': 'off',
        'render-blocking-resources': 'off',
        'unminified-javascript': 'off',
        'unminified-css': 'off',
        'byte-efficiency': 'off',
      },
    },
    upload: {
      target: 'temporary-public-storage',
    },
  },
}
