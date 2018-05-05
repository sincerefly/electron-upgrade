var fs = require('fs');
var request = require('request');
var progress = require('request-progress');
 
function download_package() {

    // The options argument is optional so you can omit it
    progress(request('https://cmake.org/files/v3.11/cmake-3.11.0-win64-x64.msi'), {
        // throttle: 2000,                    // Throttle the progress event to 2000ms, defaults to 1000ms
        // delay: 1000,                       // Only start to emit after 1000ms delay, defaults to 0ms
        // lengthHeader: 'x-transfer-length'  // Length header to use, defaults to content-length
    })
    .on('progress', function (state) {
        // The state is an object that looks like this:
        // {
        //     percent: 0.5,               // Overall percent (between 0 to 1)
        //     speed: 554732,              // The download speed in bytes/sec
        //     size: {
        //         total: 90044871,        // The total payload size in bytes
        //         transferred: 27610959   // The transferred payload size in bytes
        //     },
        //     time: {
        //         elapsed: 36.235,        // The total elapsed seconds since the start (3 decimals)
        //         remaining: 81.403       // The remaining seconds to finish (3 decimals)
        //     }
        // }
        console.log('progress', state);
    })
    .on('error', function (err) {
        // Do something with err
    })
    .on('end', function () {
        // Do something after request finishes
    })
    .pipe(fs.createWriteStream('cmake-3.11.0-win64-x64.msi'));

}


exports.download_package = download_package;