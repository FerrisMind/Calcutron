

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_layout.svelte.js')).default;
export const imports = ["_app/immutable/nodes/0.Bo4Qs7rJ.js","_app/immutable/chunks/NZTpNUN0.js","_app/immutable/chunks/CKJqsRyG.js"];
export const stylesheets = ["_app/immutable/assets/0.Cc1ufJnA.css"];
export const fonts = [];
