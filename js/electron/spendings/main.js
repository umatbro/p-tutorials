const electron = require('electron');
const url = require('url');
const path = require('path');

const {app, BrowserWindow, Menu, ipcMain} = electron;

let mainWindow;
let addWindow;

// listen for the app to be ready
app.on('ready', () => {
	mainWindow = new BrowserWindow({});
	// load html file into the window
	mainWindow.loadURL(url.format({
	  pathname: path.join(__dirname, 'mainWindow.html'),
	  protocol: 'file:',
	  slashes: true
	}));

	// build menu from template
	const mainMenu = Menu.buildFromTemplate(mainMenuTemplate);
	Menu.setApplicationMenu(mainMenu);

	// quit app when closed
	mainWindow.on('closed', () => {
		app.quit();
	});

});


// handle create add window
function createAddWindow() {
	addWindow = new BrowserWindow({
		width: 300,
		height: 200,
		title: 'Shopping list item',
	});
	addWindow.loadURL(url.format({
		pathname: path.join(__dirname, 'addWindow.html'),
		protocol: 'file:',
		slashes: true
	}));
}

// catch item:add
ipcMain.on('item:add', (event, item) => {
	mainWindow.webContents.send('item:add', item);
	addWindow.close();
});


// create menu template
const mainMenuTemplate = [
	{
		label: 'File',
		submenu: [
			{
				label: 'Add Item',
				click() {
					createAddWindow();
				}
			},
			{
				label: 'Clear items',
			},
			{
				label: 'Quit',
				accelerator: process.platform == 'darwin' ? 'Command+Q' : 'Ctrl+Q',
				click() {
					app.quit();
				},
			},
		]
	},
];

// add developer tools if not in prod
if (process.env.NODE_ENV !== 'production') {
	mainMenuTemplate.push({
		label: 'Developer tools',
		submenu: [
			{
				label: 'Toggle DevTools',
				accelerator: process.platform === 'darwin' ? 'Command+I' : 'Ctrl+I',
				click(item, focusedWindow) {
					focusedWindow.toggleDevTools();
				}
			},
			{
				role: 'reload',
			}
		]
	});
}
