async function setup() {
    document.getElementById("mlcPath").value = await readConfigJSON("mlcPath");
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
