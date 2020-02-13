const gilad2 = new Promise(resolve => resolve('Gilad'));
// const error = new Promise((resolve, reject) => reject(new Error('Error')));

(async ()=> await gilad2)()
    .then(resolve => console.log(resolve))
    .catch(error => console.log(error));