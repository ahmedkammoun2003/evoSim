import("./index.js").catch(err => {
    console.error("Error importing `index.js`:", err)
});

console.log("index.js is loaded");
