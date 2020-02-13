const axios4 = require('axios').default;

const p2 : Promise<string> = axios4.get('https://www.youtube.com/', {timeout:0.01} /* or external plugin https://www.npmjs.com/package/promise-timeout */);

(async ()=> await p2)()
    .then(resolve => console.log(resolve))
    .catch(error => console.log(error))
    .then(()=> console.log("anyway"));
