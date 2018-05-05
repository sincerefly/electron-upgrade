
const BrowserWindow = require('electron').remote.BrowserWindow;
const path = require('path');

/**
 * set mask
 */
function set_mask() {
    var classVal = document.getElementById("mask").getAttribute("class");
    if (classVal == "") {
      classVal = classVal.concat("overlay");
      document.getElementById("mask").setAttribute("class", classVal);
    }
  }
  
  /**
   * remove mask
   */
  function remove_mask() {
    var classVal = document.getElementById("mask").getAttribute("class");
    classVal = classVal.replace("overlay", "");
    document.getElementById("mask").setAttribute("class", classVal);
  }
  
  /**
   * create child window
   */
  function create_sub_window(html_name, main_position) {

    let main_x = main_position[0];
    let main_y = main_position[1];
  
    // 设置遮罩
    set_mask();
  
    const modalPath = path.join('file://', __dirname + '/subpage', html_name);
    let win = new BrowserWindow({ 
      width: 480, 
      height: 360,
      x: main_x + 80,
      y: main_y + 60,
      autoHideMenuBar: true,
      frame: false,
      resizable: false,
      transparent: true,
      useContentSize: true
    });
    
    // top
    win.setAlwaysOnTop(true)
    
    win.on('close', function () { 
      // remove mask if one window exist
      let window_count = BrowserWindow.getAllWindows()
      .filter(b => {
        return b.isVisible()
      })
      .length
  
      console.log("windows count: " + window_count);
      if (window_count == 1) {
        remove_mask();
      }
      win = null
    })
    
    win.loadURL(modalPath)

    win.show()

    return win
  }
  
  
  /**
   *  alert setting window
   */
  const settingBtn = document.getElementById('setting');
  settingBtn.addEventListener('click', function (event) {

    let mainWindow = BrowserWindow.getFocusedWindow()
    let main_position = mainWindow.getPosition();

    create_sub_window("setting.html", main_position);
  
  });

