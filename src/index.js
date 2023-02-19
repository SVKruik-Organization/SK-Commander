const { app, BrowserWindow, shell } = require('electron');
const path = require('path');

const createWindow = () => {
    const win = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            nodeIntegration: true,
            nodeIntegrationInWorker: true,
            sandbox: false,
            spellcheck: true,
        },
        autoHideMenuBar: true,
        webPreferences: {
            preload: path.join(__dirname, '/preload.js'),
        },
    });

    win.loadFile('./frontend/html/statistics.html');
    win.webContents.setWindowOpenHandler(({ url }) => {
        shell.openExternal(url);
        return { action: 'deny' };
    });
};

app.whenReady().then(() => {
    createWindow();
});

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') app.quit();
});
