// pausable
const gilad3 = new Promise(resolve => /* resumer */ resolve('Gilad'));
const error = new Promise((resolve, reject) => reject(new Error('Error')));

(async ()=> await gilad3)()
    .then(resolve => console.log(resolve))
    .catch(error => console.log(error));

// pause
(async ()=> await error)()
    .then(resolve => console.log(resolve))
    .catch(error => console.log(error));