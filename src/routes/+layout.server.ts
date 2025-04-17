export function load() {
	console.log(process.env.PUBLIC_API_URL ?? 'http://localhost/api (DEFAULT, NO ENV FOUND!)');
	return {
		apiUrl: process.env.PUBLIC_API_URL ?? 'http://localhost/api'
	};
}