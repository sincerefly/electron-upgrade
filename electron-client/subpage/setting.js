const fs = require('fs');
const {ipcRenderer} = require('electron');

// Get update server address from config
(function() {
  let config = JSON.parse(fs.readFileSync('conf/config.json', 'utf8'));
  console.log(config);
  let update_ip = config['update_ip'];
  let update_port = config['update_port'];
  document.getElementById("updateip").value = update_ip;
  document.getElementById("updateport").value = update_port;
})();

// When save address
let saveBtn = document.getElementById("save");
saveBtn.addEventListener('click', () => {
  let update_ip = document.getElementById("updateip").value;
  let update_port = document.getElementById("updateport").value;
  console.log(update_ip);
  ipcRenderer.send('save-address', {"update_ip": update_ip, "update_port": update_port});
});

// Close setting page when success
ipcRenderer.on('save-address-status', (status) => {
  if (status) {
    window.close();        
  } else {
    console.log("save address failed");
  }
});