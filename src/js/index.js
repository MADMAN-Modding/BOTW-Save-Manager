async function setup() {
    document.getElementById("mlcPath").value = await readConfigJSON("mlcPath");
}

async function updateMLC() {
    pushNotification("Updating MLC Path");
    let path = document.getElementById("mlcPath").value;

    writeConfigJSON("mlcPath", path);

    pushNotification("Updated MLC Path")
}

setup();
