const axios3 = require('axios').default;
const p = axios3.get('https://www.youtube.com/');
(async () => await p)()
    .then(resolve => console.log(resolve))
    .catch(error => console.log(error))
    .then(() => console.log("anyway"));
