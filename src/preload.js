const { contextBridge } = require('electron');
const { mysql } = require('mysql2');

contextBridge.exposeInMainWorld('versions', {
  node: () => process.versions.node,
  chrome: () => process.versions.chrome,
  electron: () => process.versions.electron,
});

const database = mysql.createPool({
  host: process.env.HOST,
  user: process.env.USER,
  database: process.env.DATABASE,
  password: process.env.PASSWORD
});

database.promise()
  .execute("SHOW databases")
  .then(() => {
    console.log("\nDatabase connection established.\n");
  }).catch((err) => {
    console.log("[ERROR] Connecting to the database went wrong.", err);
    process.exit();
  });

console.log("Preload Script Loaded.");
