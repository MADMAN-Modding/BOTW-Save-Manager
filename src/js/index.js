async function setup() {
    mlcPath = await readConfigJSON("mlcPath");
    
    
    if (mlcPath == "NOT_SET") {
        let time = new Date();

        let startTime = time.getTime();

        pushNotification("Searching for mlc01 path, this could take a while...")
        await invoke("start_search").then((value) => 
            {
                time = new Date();
                // Uses a bitwise operator to remove the decimals then divides by 1000 to convert to seconds
                let duration = ((time.getTime() - startTime) / 1000) | 0;
                
                pushNotification(`${value} in ${duration} seconds`);
                writeConfigJSON("mlcPath", value);
                mlcPath = value;
            });
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

setup();
