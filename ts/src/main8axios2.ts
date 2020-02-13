const axios2 = require('axios').default;

(/*pausable*/ async () => {
    try {
        const resolve = /* pause */ await axios2.get('/user?ID=12345') /* resumer */;
        console.log(resolve);
    } catch (reject) {
        console.error(reject);
    }
})();