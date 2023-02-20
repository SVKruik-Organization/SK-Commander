const mysql = require('mysql2')
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