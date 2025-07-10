export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.svg","Fonts/bold/Phosphor-Bold.svg","Fonts/bold/Phosphor-Bold.ttf","Fonts/bold/Phosphor-Bold.woff","Fonts/bold/Phosphor-Bold.woff2","Fonts/bold/selection.json","Fonts/bold/style.css","Fonts/duotone/Phosphor-Duotone.svg","Fonts/duotone/Phosphor-Duotone.ttf","Fonts/duotone/Phosphor-Duotone.woff","Fonts/duotone/Phosphor-Duotone.woff2","Fonts/duotone/selection.json","Fonts/duotone/style.css","Fonts/fill/Phosphor-Fill.svg","Fonts/fill/Phosphor-Fill.ttf","Fonts/fill/Phosphor-Fill.woff","Fonts/fill/Phosphor-Fill.woff2","Fonts/fill/selection.json","Fonts/fill/style.css","Fonts/light/Phosphor-Light.svg","Fonts/light/Phosphor-Light.ttf","Fonts/light/Phosphor-Light.woff","Fonts/light/Phosphor-Light.woff2","Fonts/light/selection.json","Fonts/light/style.css","Fonts/regular/Phosphor.svg","Fonts/regular/Phosphor.ttf","Fonts/regular/Phosphor.woff","Fonts/regular/Phosphor.woff2","Fonts/regular/selection.json","Fonts/regular/style.css","Fonts/thin/Phosphor-Thin.svg","Fonts/thin/Phosphor-Thin.ttf","Fonts/thin/Phosphor-Thin.woff","Fonts/thin/Phosphor-Thin.woff2","Fonts/thin/selection.json","Fonts/thin/style.css"]),
	mimeTypes: {".svg":"image/svg+xml",".ttf":"font/ttf",".woff":"font/woff",".woff2":"font/woff2",".json":"application/json",".css":"text/css"},
	_: {
		client: {start:"_app/immutable/entry/start.vIcuJMnC.js",app:"_app/immutable/entry/app.xPd2jQ-P.js",imports:["_app/immutable/entry/start.vIcuJMnC.js","_app/immutable/chunks/5k-Nodat.js","_app/immutable/chunks/BQJ8lqPq.js","_app/immutable/chunks/CKJqsRyG.js","_app/immutable/entry/app.xPd2jQ-P.js","_app/immutable/chunks/CKJqsRyG.js","_app/immutable/chunks/BQJ8lqPq.js","_app/immutable/chunks/NZTpNUN0.js","_app/immutable/chunks/B2hT9mK3.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js'))
		],
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			}
		],
		prerendered_routes: new Set([]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
