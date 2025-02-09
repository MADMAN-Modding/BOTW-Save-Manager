/**
 * Makes all the divs for showing the available saves
 * @returns {void}
 * @async
 */
async function makeListings() {
    let folders = await invoke("get_folders_in_dir", {"path": await invoke("get_data_dir")});

    let backups = document.getElementById("backups");

    backups.innerHTML = "";

    folders.forEach(async folder => {
        console.log(folder);
        let img = await getImage(folder);
        backups.innerHTML += `
            <div class="backup">
                <span>${folder}</span>
                <img id="saveIcon" src="${img}" />
            </div>`;
    });
    
}

async function newBackup() {
    pushNotification("Creating backup");

    let backupName = document.getElementById("backupName").value;

    invoke("new_backup", {"name": backupName}).catch(error => pushNotification(error));

    pushNotification("Backup Complete");

    makeListings();
}

/**
 * Retrieves the bytes of the code directory image.
 * 
 * Creates a `Uint8Array` from the retrieved bytes.
 * 
 * Converts the `Uint8Array` into a `Blob` with the MIME type `image/png`.
 * 
 * Create of an `ObjectURL` of the blob
 * 
 * Sets the `esportsLogo` document to the `ObjectURL`
 * @async
*/
async function getImage(path) {
    let bytes;
    
    bytes = await invoke('get_image_bytes', {"path": path})
        .then((value) => bytes = value)
        .catch((error) => {pushNotification(error); return;});

    bytes = new Uint8Array(bytes);

    const blob = new Blob([bytes], { type: "image/png" });
    const imageURL = URL.createObjectURL(blob);

    return imageURL;
}

makeListings();