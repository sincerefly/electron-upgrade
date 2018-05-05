const electron = require('electron')
const app = electron.app
const { ipcMain } = require('electron');
const fs = require('fs');

// Listening ipc message (main process)

/**
 * relaunch app
 */
ipcMain.on('relaunch', (_) => {
  console.log("relaunch main");
  app.relaunch()
  app.exit()
});

/**
 * save address
 */
ipcMain.on('save-address', (event, address) => {

  let config = JSON.parse(fs.readFileSync('conf/config.json', 'utf8'));
  config['update_ip'] = address['update_ip'];
  config['update_port'] = address['update_port'];
  
  fs.writeFile('conf/config.json', JSON.stringify(config), (err) => {
    if (err) {
      event.sender.send('save-address-status', false);
    } else {
      event.sender.send('save-address-status', true);
    }
  });
});