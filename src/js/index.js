async function setup() {
    mlcPath = await readConfigJSON("mlcPath");
    
    
    if (mlcPath == "NOT_SET") {
        scanMLC();
        return;
    }
    
    document.getElementById("mlcPath").value = mlcPath;
}

/**
 * Updates the MLC path in the config.json file.
 * 
 * Reads the value from the `mlcPath` input element.
 * 
 * Calls the `writeConfigJSON` function from jsonHandler.js, passing the arguments `mlcPath` and the value from the `mlcPath` input element.
 * 
 * Pushes a notification that the MLC path has been updated.
 * 
 * @async
 * @returns {void}
 */
async function updateMLC() {
    pushNotification("Updating MLC Path");
    let path = document.getElementById("mlcPath").value;

    writeConfigJSON("mlcPath", path);

    pushNotification("Updated MLC Path")
}

/**
 * Scans for the MLC path.
 * 
 * Pushes a notification that the MLC path is being searched for.
 * 
 * Pushes a notification that the MLC path has been found.
 * 
 * Writes to the config and returns the value of the promise.
 * 
 * Updates the value of the `mlcPath` input element to the value from the promise.
 * 
 * @async
 * @returns {String}
*/
async function scanMLC() {
    let time = new Date();

    let startTime = time.getTime();

    pushNotification("Searching for mlc01 path, this could take a while...")
    await invoke("start_search").then((value) => 
        {
            time = new Date();
            // Uses a bitwise operator to remove the decimals then divides by 1000 to convert to seconds
            let duration = ((time.getTime() - startTime) / 1000) | 0;
            
            pushNotification(`Found ${value} in ${duration} seconds`);
            writeConfigJSON("mlcPath", value);
            return value;
        });
    document.getElementById("mlcPath").value = mlcPath;
}

setup();
