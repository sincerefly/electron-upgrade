const fs = require('fs');
const {ipcRenderer} = require('electron');

// save package info
var info = "";

let checkBtn = document.getElementById("check");
checkBtn.addEventListener('click', () => {
  info = {
    "md5": "e10adc3949ba59abbe56e057f20f883e",
    "from": "1.0.1",
    "to": "1.0.2",
    "desc": "Fix Some Bug.",
    "url": "http://127.0.0.1:8000/download/V1.0.2/package.zip",
    "ready": true
  }
  let currentVersion = document.getElementById("currentVersion").innerHTML;
  // let latestVersion = document.getElementById("latestVersion").innerHTML;

  if (currentVersion == info["to"]) {
    document.getElementById("lv").style.display="";
    document.getElementById("status").style.display="";
    document.getElementById("message").innerHTML = "Already the latest version";
  } else if (currentVersion == info["from"]) {
    document.getElementById("lv").style.display="";
    document.getElementById("latestVersion").innerHTML = info["to"];
    document.getElementById("note").innerHTML = info["desc"];
    document.getElementById("rn").style.display="";
    document.getElementById("status").style.display="none";
  }
});

let upgradeBtn = document.getElementById("upgrade");
upgradeBtn.addEventListener('click', () => {

  console.log(info);

  if (info == "") {
    document.getElementById("status").style.display="";
    document.getElementById("message").innerHTML = "Please check first";
    return;
  }

  let currentVersion = document.getElementById("currentVersion").innerHTML;
  let latestVersion = document.getElementById("latestVersion").innerHTML;
  if (currentVersion == latestVersion) {
    document.getElementById("status").style.display="";
    document.getElementById("message").innerHTML = "";
    document.getElementById("message").innerHTML = "Already the latest version";
  } else {
    document.getElementById("cv").style.display="";
    document.getElementById("lv").style.display="";
    document.getElementById("rn").style.display="none";
    document.getElementById("status").style.display="";
  }

  // download file
  ipcRenderer.send('download-package', info["url"]);

});


ipcRenderer.on('progress', (event, state) => {

  let percent = state['percent'];

  if (percent != 1) {
    let percent = (state['percent'] * 100).toFixed(2).toString() + '%';
    document.getElementById("message").innerHTML = percent;
  } else if (percent == -1) {
    document.getElementById("message").innerHTML = "网络错误";
  } else {
    document.getElementById("message").innerHTML = "100%";
    // TODO: unzip
  }
});