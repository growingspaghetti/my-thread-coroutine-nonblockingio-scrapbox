import ky from 'ky';

(async () => {
	const parsed = await ky.post('https://jsonplaceholder.typicode.com/todos/1', {json: {foo: true}}).json();

	console.log(parsed);
	//=> `{data: '🦄'}`
})();


