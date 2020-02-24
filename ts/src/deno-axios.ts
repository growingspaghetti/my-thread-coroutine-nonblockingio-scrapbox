
import axios from "https://gist.githubusercontent.com/DanielRamosAcosta/2f773d815f5434f185c59aec1bab418c/raw/a442cdd8699e39ab9855cbaa571a79049a7b67d4/axios.ts"

// Make a request for a user with a given ID
axios.get('http://jsonplaceholder.typicode.com/users/1')
  .then(response => {
    // handle success
    console.log("User name:", response.data.name);
  })
  .catch(error => {
    // handle error
    console.error("error:", error);
  })
