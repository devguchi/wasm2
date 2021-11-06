const rust = import("../pkg/index.js");
rust.then(m => {
    const result = m.add(1,2); 
    alert(`1+2=${result}`);
}).catch(console.error);

