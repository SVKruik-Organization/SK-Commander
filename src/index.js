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

    win.loadFile('./frontend/html/login.html');
    win.webContents.setWindowOpenHandler(({ url }) => {
        shell.openExternal(url);
        return { action: 'deny' };
    });
};

app.setUserTasks([
    {
        program: process.execPath,
        arguments: '--new-window',
        iconPath: process.execPath,
        iconIndex: 0,
        title: 'New Window',
        description: 'Opens a new window'
    }
]);

app.whenReady().then(() => {
    createWindow();
});

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') app.quit();
});
