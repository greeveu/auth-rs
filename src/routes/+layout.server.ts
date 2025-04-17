export function load() {
	console.log(process.env.PUBLIC_API_URL);
	return {
		apiUrl: process.env.PUBLIC_API_URL ?? 'http://localhost/api'
	};
}