const axios = require('axios').default;
axios.get('https://www.youtube.com/')
    // resolve
    .then(function (response) {
    console.log(response);
})
    // reject
    .catch(function (error) {
    console.log(error);
})
    // finally
    .then(function () {
    console.log("defer");
});
