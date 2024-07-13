const cacheName = 'egui-template-pwa';
const filesToCache = [
    './',
    './index.html',
    './biobots.js',
    './biobots_bg.wasm',
];

/* Start the service worker and cache all the app's content */
self.addEventListener('install', function (e) {
    e.waitUntil(
        caches.open(cacheName).then(cache => cache.addAll(filesToCache))
    );
});

/* Serve cached content when offline */
self.addEventListener('fetch', function (e) {
    e.respondWith(
        caches.match(e.request).then(response => response || fetch(e.request))
    );
});
