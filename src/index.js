const { app, BrowserWindow, shell, session } = require('electron');
const mysql = require('mysql2');

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

let win;
const createWindow = () => {
    win = new BrowserWindow({
        width: 800,
        height: 600,
        autoHideMenuBar: true,
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false,
            sandbox: true
        },
    });

    win.loadFile('./www/index.html');
};

app.on('web-contents-created', (event, contents) => {
    contents.setWindowOpenHandler(({ url }) => {
        if (isSafeForExternalOpen(url)) {
            setImmediate(() => {
                shell.openExternal(url)
            })
        };
        return { action: 'deny' }
    });
});

app.whenReady().then(() => {
    createWindow();

    session
        .fromPartition('some-partition')
        .setPermissionRequestHandler((webContents, permission, callback) => {
            const parsedUrl = new URL(webContents.getURL())
            if (permission === 'notifications') {
                callback(true)
            }

            if (parsedUrl.protocol !== 'https:') {
                return callback(false)
            }
        });
});

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') app.quit();
});

