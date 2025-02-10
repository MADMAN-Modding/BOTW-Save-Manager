/**
 * Makes all the divs for showing the available saves
 * @returns {void}
 * @async
 */
async function makeListings() {
    let folders = await invoke("get_folders_in_dir", { "path": await invoke("get_data_dir") });

    let saves = document.getElementById("saves");

    saves.innerHTML = "";

    let saveElements = await Promise.all(folders.map(async folder => {
        let img = await getImage(folder);
        console.log(folder);

        return `
            <div class="save">
                <span>${folder}</span>
                <img class="controlIcons" id="load" onclick="loadSave('${folder}')" src="images/load.png"/>
                <img class="controlIcons" id="delete" onclick="removeSave('${folder}')" src="images/delete.png"/>
                <img id="saveIcon" src="${img}"/>
            </div>`;
    }));

    saves.innerHTML = saveElements.join(""); // Update once to prevent layout thrashing
}


async function newSave() {
    pushNotification("Creating save");

    let saveName = document.getElementById("saveName").value;

    console.log(saveName);

    let illegalChars = ["\"", "\\", "/"];

    if (saveName == "") {
        pushNotification("No path provided");
        return;
    }
    
    for (let index = 0; index < illegalChars.length; index++) {
        const char = illegalChars[index];
        
        if (saveName.includes(char)) {
            await pushNotification(`Illegal Char: ${char}`);
            return;
        }
    }

    invoke("new_save", {"name": saveName}).catch(error => pushNotification(error));

    pushNotification("Save Complete");

    makeListings();
}

async function removeSave(path) {
    console.log(path);
    
    pushNotification("Removing Save: " + path);

    await invoke("remove_save", {"path": path});

    pushNotification("Removed Save: " + path);

    makeListings()
}

async function loadSave(path) {
    pushNotification("Loading Save: " + path);

    await invoke("load_save", {"path": path});

    pushNotification("Loaded Save: " + path);
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